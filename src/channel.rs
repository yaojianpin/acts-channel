use crate::{
    acts_service_client::*,
    model::{self, Package},
    utils, ActionResult, Message, MessageOptions, Vars,
};
use serde::{de::DeserializeOwned, Serialize};
use std::str::FromStr;
use tokio_stream::StreamExt;
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
    /// auto ack message when receiving by client
    pub ack: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ActsChannel {
    client: ActsServiceClient<Channel>,
    auto_ack: bool,
}

impl ActsChannel {
    pub async fn connect(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let addr = Endpoint::from_str(url)?;
        let client = ActsServiceClient::connect(addr).await?;
        Ok(Self {
            client,
            auto_ack: true,
        })
    }

    /// subscribe the server message
    pub async fn subscribe<F: Fn(&model::Message) + Send + Sync + 'static>(
        &mut self,
        client_id: &str,
        on_message: F,
        options: &ActsOptions,
    ) {
        let mut client = self.client.clone();
        if let Some(auto_ack) = options.ack {
            self.auto_ack = auto_ack;
        }
        self.on_message(&mut client, client_id, on_message, options)
            .await;
    }

    pub async fn deploy(
        &mut self,
        model: &str,
        mid: Option<&str>,
    ) -> Result<ActionResult<bool>, Status> {
        let mut options = Vars::new();
        options.set("model", model.to_string());

        if let Some(mid) = mid {
            options.set("mid", mid.to_string());
        }

        self.send("model:deploy", options).await
    }

    pub async fn publish(&mut self, package: &Package) -> Result<ActionResult<bool>, Status> {
        let options = Vars::new()
            .with("id", package.id.clone())
            .with("name", package.name.clone())
            .with("body", package.body.clone());

        self.send("pack:publish", options).await
    }

    pub async fn start(&mut self, id: &str, vars: Vars) -> Result<ActionResult<String>, Status> {
        let options = Vars::new().with("id", id).extend(&vars);
        let mut ret = ActionResult::<String>::begin();
        let resp = self
            .client
            .send(Request::new(crate::Message {
                name: "proc:start".to_string(),
                seq: utils::create_seq(),
                ack: None,
                data: Some(options.to_bytes()),
            }))
            .await?;

        let data = resp
            .into_inner()
            .data
            .map(|v| serde_json::from_slice(&v).unwrap());
        ret.data = data;
        ret.end()
    }

    pub async fn ack(&mut self, ack_id: &str) -> Result<ActionResult<()>, Status> {
        self.send_with_ack(
            "msg:ack",
            Vars::new().with("id", ack_id),
            Some(ack_id.to_string()),
        )
        .await
    }

    pub async fn send<T>(&mut self, name: &str, data: Vars) -> Result<ActionResult<T>, Status>
    where
        T: Serialize + DeserializeOwned,
    {
        self.send_with_ack(name, data, None).await
    }

    async fn send_with_ack<T>(
        &mut self,
        name: &str,
        data: Vars,
        ack: Option<String>,
    ) -> Result<ActionResult<T>, Status>
    where
        T: Serialize + DeserializeOwned,
    {
        let mut ret = ActionResult::begin();
        let resp = self
            .client
            .send(Request::new(Message {
                name: name.to_string(),
                seq: utils::create_seq(),
                ack,
                data: Some(data.to_bytes()),
            }))
            .await?;
        ret.data = resp
            .into_inner()
            .data
            .map(|v| serde_json::from_slice(&v).unwrap());
        ret.end()
    }

    async fn on_message(
        &self,
        client: &mut ActsServiceClient<Channel>,
        client_id: &str,
        handle: impl Fn(&model::Message) + Send + Sync + 'static,
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
        let auto_ack = self.auto_ack;
        tokio::spawn(async move {
            let mut chan = chan.clone();
            while let Some(item) = stream.next().await {
                if !item.is_err() {
                    let m: Message = item.unwrap().into();
                    let message = serde_json::from_slice(m.data()).unwrap();
                    let seq = &m.seq;

                    if auto_ack {
                        // auto ack the message
                        match chan.ack(seq).await {
                            Ok(_) => {
                                handle(&message);
                            }
                            Err(err) => {
                                println!("on_message err:{:?}", err);
                            }
                        }
                    } else {
                        handle(&message);
                    }
                }
            }
        });
    }
}
