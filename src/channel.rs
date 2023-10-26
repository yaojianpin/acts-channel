use crate::{
    acts_service_client::*, model::Message, ActionOptions, ActionResult, MessageOptions, Vars,
};
use futures::StreamExt;
use std::str::FromStr;
use tonic::{
    transport::{Channel, Endpoint},
    Request, Status,
};

#[derive(Debug, Clone, Default)]
pub struct ActsOptions {
    pub on_message: Option<fn(&Message)>,
    pub r#type: Option<String>,
    pub state: Option<String>,
    pub tag: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ActsChannel {
    inner: ActsServiceClient<Channel>,
}

impl ActsChannel {
    pub async fn new(
        url: &str,
        client_id: &str,
        options: &ActsOptions,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let addr = Endpoint::from_str(url)?;
        let mut client = ActsServiceClient::connect(addr).await?;

        if let Some(on_message) = options.on_message {
            Self::on_message(&mut client, client_id, on_message, options).await;
        }

        Ok(Self { inner: client })
    }

    pub async fn deploy(&mut self, mid: &str, model: &str) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("model".to_string(), model.to_string());
        options.insert_str("mid".to_string(), mid.to_string());

        self.do_action("deploy", &options).await
    }

    pub async fn rm(&mut self, mid: &str) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("mid".to_string(), mid.to_string());

        self.do_action("rm", &options).await
    }

    pub async fn start(
        &mut self,
        mid: &str,
        uid: &str,
        vars: &Vars,
    ) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("mid".to_string(), mid.clone());
        options.insert_str("uid".to_string(), uid.clone());
        options.extend(vars);

        let resp = self
            .inner
            .action(Request::new(ActionOptions {
                name: "start".to_string(),
                options: Some(options.prost_vars()),
            }))
            .await?;

        Ok(resp.into_inner())
    }

    pub async fn submit(
        &mut self,
        pid: &str,
        tid: &str,
        uid: &str,
        vars: &Vars,
    ) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("pid".to_string(), pid.clone());
        options.insert_str("tid".to_string(), tid.clone());
        options.insert_str("uid".to_string(), uid.clone());
        options.extend(vars);

        self.do_action("submit", &options).await
    }

    pub async fn complete(
        &mut self,
        pid: &str,
        tid: &str,
        uid: &str,
        vars: &Vars,
    ) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("pid".to_string(), pid.clone());
        options.insert_str("tid".to_string(), tid.clone());
        options.insert_str("uid".to_string(), uid.clone());
        options.extend(vars);

        self.do_action("complete", &options).await
    }

    pub async fn back(
        &mut self,
        pid: &str,
        tid: &str,
        uid: &str,
        to: &str,
        vars: &Vars,
    ) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("pid".to_string(), pid.clone());
        options.insert_str("tid".to_string(), tid.clone());
        options.insert_str("uid".to_string(), uid.clone());
        options.insert_str("to".to_string(), to.clone());
        options.extend(vars);

        self.do_action("back", &options).await
    }

    pub async fn cancel(
        &mut self,
        pid: &str,
        tid: &str,
        uid: &str,
        vars: &Vars,
    ) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("pid".to_string(), pid.clone());
        options.insert_str("tid".to_string(), tid.clone());
        options.insert_str("uid".to_string(), uid.clone());
        options.extend(vars);

        self.do_action("cancel", &options).await
    }

    async fn do_action(&mut self, name: &str, options: &Vars) -> Result<ActionResult, Status> {
        let resp = self
            .inner
            .action(Request::new(ActionOptions {
                name: name.to_string(),
                options: Some(options.prost_vars()),
            }))
            .await?;

        Ok(resp.into_inner())
    }

    async fn on_message(
        client: &mut ActsServiceClient<Channel>,
        client_id: &str,
        handle: impl Fn(&Message) + Send + Sync + 'static,
        options: &ActsOptions,
    ) {
        let request = tonic::Request::new(MessageOptions {
            client_id: client_id.to_string(),
            r#type: options
                .r#type
                .as_ref()
                .map(|i| i.as_str())
                .unwrap_or("*")
                .to_string(),
            state: options
                .state
                .as_ref()
                .map(|i| i.as_str())
                .unwrap_or("*")
                .to_string(),
            tag: options
                .tag
                .as_ref()
                .map(|i| i.as_str())
                .unwrap_or("*")
                .to_string(),
        });
        let mut stream = client.on_message(request).await.unwrap().into_inner();

        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                if !item.is_err() {
                    let m: Message = item.unwrap().into();
                    handle(&m);
                }
            }
        });
    }
}
