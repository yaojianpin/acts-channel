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
    pub r#type: Option<String>,
    pub state: Option<String>,
    pub tag: Option<String>,
    pub key: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ActsChannel {
    client: ActsServiceClient<Channel>,
}

impl ActsChannel {
    pub async fn new(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let addr = Endpoint::from_str(url)?;
        let client = ActsServiceClient::connect(addr).await?;
        Ok(Self { client })
    }

    /// subscribe the server message
    pub async fn sub<F: Fn(&Message) + Send + Sync + 'static>(
        &mut self,
        client_id: &str,
        on_message: F,
        options: &ActsOptions,
    ) {
        let mut client = self.client.clone();
        self.on_message(&mut client, client_id, on_message, options)
            .await;
    }

    pub async fn deploy(&mut self, model: &str, mid: Option<&str>) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("model".to_string(), model.to_string());

        if let Some(mid) = mid {
            options.insert_str("mid".to_string(), mid.to_string());
        }

        self.do_action("deploy", &options).await
    }

    pub async fn rm(&mut self, options: &Vars) -> Result<ActionResult, Status> {
        self.do_action("rm", &options).await
    }

    pub async fn start(&mut self, mid: &str, vars: &Vars) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("mid".to_string(), mid);
        options.extend(vars);
        let mut ret = ActionResult::begin();
        let resp = self
            .client
            .action(Request::new(ActionOptions {
                name: "start".to_string(),
                options: Some(options.prost_vars()),
            }))
            .await?;
        ret.data = resp.into_inner();
        ret.end()
    }

    pub async fn push(
        &mut self,
        pid: &str,
        tid: &str,
        vars: &Vars,
    ) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("pid".to_string(), pid);
        options.insert_str("tid".to_string(), tid);
        options.extend(vars);

        self.do_action("push", &options).await
    }

    pub async fn remove(
        &mut self,
        pid: &str,
        tid: &str,
        vars: &Vars,
    ) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("pid".to_string(), pid);
        options.insert_str("tid".to_string(), tid);
        options.extend(vars);

        self.do_action("remove", &options).await
    }

    pub async fn submit(
        &mut self,
        pid: &str,
        tid: &str,
        vars: &Vars,
    ) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("pid".to_string(), pid);
        options.insert_str("tid".to_string(), tid);
        options.extend(vars);

        self.do_action("submit", &options).await
    }

    pub async fn complete(
        &mut self,
        pid: &str,
        tid: &str,
        vars: &Vars,
    ) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("pid".to_string(), pid);
        options.insert_str("tid".to_string(), tid);
        options.extend(vars);

        self.do_action("complete", &options).await
    }

    pub async fn back(
        &mut self,
        pid: &str,
        tid: &str,
        to: &str,
        vars: &Vars,
    ) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("pid".to_string(), pid);
        options.insert_str("tid".to_string(), tid.to_string());
        options.insert_str("to".to_string(), to.to_string());
        options.extend(vars);

        self.do_action("back", &options).await
    }

    pub async fn cancel(
        &mut self,
        pid: &str,
        tid: &str,
        vars: &Vars,
    ) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("pid".to_string(), pid.to_string());
        options.insert_str("tid".to_string(), tid.to_string());
        options.extend(vars);

        self.do_action("cancel", &options).await
    }

    pub async fn skip(
        &mut self,
        pid: &str,
        tid: &str,
        vars: &Vars,
    ) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("pid".to_string(), pid.to_string());
        options.insert_str("tid".to_string(), tid.to_string());
        options.extend(vars);

        self.do_action("skip", &options).await
    }

    pub async fn error(
        &mut self,
        pid: &str,
        tid: &str,
        vars: &Vars,
    ) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("pid".to_string(), pid.to_string());
        options.insert_str("tid".to_string(), tid.to_string());
        options.extend(vars);

        self.do_action("error", &options).await
    }

    async fn ack(&mut self, id: &str) -> Result<ActionResult, Status> {
        let mut options = Vars::new();
        options.insert_str("id".to_string(), id.to_string());
        self.do_action("ack", &options).await
    }

    pub async fn do_action(&mut self, name: &str, options: &Vars) -> Result<ActionResult, Status> {
        let mut ret = ActionResult::begin();
        let resp = self
            .client
            .action(Request::new(ActionOptions {
                name: name.to_string(),
                options: Some(options.prost_vars()),
            }))
            .await?;
        ret.data = resp.into_inner();
        ret.end()
    }

    async fn on_message(
        &self,
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

            key: options
                .key
                .as_ref()
                .map(|i| i.as_str())
                .unwrap_or("*")
                .to_string(),
        });
        let mut stream = client.on_message(request).await.unwrap().into_inner();
        let chan = self.clone();
        tokio::spawn(async move {
            let mut chan = chan.clone();
            while let Some(item) = stream.next().await {
                if !item.is_err() {
                    let m: Message = item.unwrap().into();
                    if let Ok(_) = chan.ack(&m.id).await {
                        handle(&m);
                    }
                }
            }
        });
    }
}
