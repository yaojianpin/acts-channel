use std::{collections::HashMap, pin::Pin, sync::Arc};

use crate::{acts_service_server::ActsService, utils, Message, MessageOptions, Vars};
use acts::{data::Package, ChannelOptions, Engine, Workflow};
use futures::Stream;
use serde::Serialize;
use tokio::sync::{
    mpsc::{self, Sender},
    Mutex,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{self, Code, Response, Status};

type MessageStream = Pin<Box<dyn Stream<Item = Result<Message, tonic::Status>> + Send + 'static>>;

fn wrap_message<T: ?Sized + Serialize>(name: &str, value: &T) -> Message {
    Message {
        name: name.to_string(),
        seq: utils::create_seq(),
        ack: None,
        data: Some(serde_json::to_vec(value).unwrap()),
    }
}

macro_rules! wrap_result {
    ($seq: expr, $name:expr, $input: expr) => {
        match $input {
            Ok(data) => {
                let mut message = wrap_message($name, &data);
                message.ack = Some($seq.to_string());
                Ok(Response::new(message))
            }
            Err(err) => {
                println!("wrap_result err= {err:?}");
                Err(Status::new(Code::Internal, err.to_string()))
            }
        }
    };
}

impl Into<acts::Vars> for &Vars {
    fn into(self) -> acts::Vars {
        let mut vars = acts::Vars::new();
        for (name, value) in self.iter() {
            vars.set(name, value);
        }

        vars
    }
}

#[derive(Clone)]
pub struct MessageClient {
    addr: String,
    sender: Sender<Result<Message, Status>>,
    options: ChannelOptions,
}

#[derive(Clone)]
pub struct GrpcServer {
    engine: Arc<Engine>,
    clients: Arc<Mutex<HashMap<String, MessageClient>>>,
}

impl MessageClient {
    fn send(&self, message: Message) {
        let msg = Ok(message);
        let client = self.clone();
        tokio::spawn(async move {
            match client.sender.send(msg).await {
                Ok(_) => {
                    println!("[OK] send to {}({})", client.addr, client.options.id);
                }
                Err(err) => {
                    println!(
                        "[ERROR] send to {}({}), error={:?}",
                        client.addr, client.options.id, err
                    );
                    // clients.remove(index);
                }
            }
        });
    }
}

impl GrpcServer {
    pub fn new() -> Self {
        let inst = Self {
            engine: Arc::new(Engine::new()),
            clients: Arc::new(Mutex::new(HashMap::new())),
        };

        inst
    }

    pub async fn init(&self) {
        let clients = self.clients.lock().await;
        for (_, client) in clients.iter() {
            let chan = self.engine.channel_with_options(&client.options);
            let c = client.clone();
            chan.on_message(move |e| {
                let m: &acts::Message = e;
                let message = wrap_message(&m.name, m);
                c.send(message);
            });
        }
    }

    fn do_action(&self, message: Message) -> Result<Response<Message>, Status> {
        let options =
            &serde_json::from_slice::<acts::Vars>(&message.data.unwrap_or_default()).unwrap();
        let name = message.name.as_str();
        let ack = message.seq.as_str();
        let executor = self.engine.executor();
        match name {
            // do act
            "act:push" => {
                let pid = options
                    .get::<String>("pid")
                    .ok_or(Status::invalid_argument("pid is required"))?;
                let tid = options
                    .get::<String>("tid")
                    .ok_or(Status::invalid_argument("tid is required"))?;
                wrap_result!(ack, name, executor.act().push(&pid, &tid, options))
            }
            "act:remove" => {
                let pid = options
                    .get::<String>("pid")
                    .ok_or(Status::invalid_argument("pid is required"))?;
                let tid = options
                    .get::<String>("tid")
                    .ok_or(Status::invalid_argument("tid is required"))?;

                wrap_result!(ack, name, executor.act().remove(&pid, &tid, options))
            }
            "act:submit" => {
                let pid = options
                    .get::<String>("pid")
                    .ok_or(Status::invalid_argument("pid is required"))?;
                let tid = options
                    .get::<String>("tid")
                    .ok_or(Status::invalid_argument("tid is required"))?;

                wrap_result!(ack, name, executor.act().submit(&pid, &tid, options))
            }
            "act:complete" => {
                let pid = options
                    .get::<String>("pid")
                    .ok_or(Status::invalid_argument("pid is required"))?;
                let tid = options
                    .get::<String>("tid")
                    .ok_or(Status::invalid_argument("tid is required"))?;

                wrap_result!(ack, name, executor.act().complete(&pid, &tid, options))
            }
            "act:abort" => {
                let pid = options
                    .get::<String>("pid")
                    .ok_or(Status::invalid_argument("pid is required"))?;
                let tid = options
                    .get::<String>("tid")
                    .ok_or(Status::invalid_argument("tid is required"))?;

                wrap_result!(ack, name, executor.act().abort(&pid, &tid, options))
            }
            "act:cancel" => {
                let pid = options
                    .get::<String>("pid")
                    .ok_or(Status::invalid_argument("pid is required"))?;
                let tid = options
                    .get::<String>("tid")
                    .ok_or(Status::invalid_argument("tid is required"))?;

                wrap_result!(ack, name, executor.act().cancel(&pid, &tid, options))
            }
            "act:back" => {
                let pid = options
                    .get::<String>("pid")
                    .ok_or(Status::invalid_argument("pid is required"))?;
                let tid = options
                    .get::<String>("tid")
                    .ok_or(Status::invalid_argument("tid is required"))?;

                wrap_result!(ack, name, executor.act().back(&pid, &tid, options))
            }
            "act:skip" => {
                let pid = options
                    .get::<String>("pid")
                    .ok_or(Status::invalid_argument("pid is required"))?;
                let tid = options
                    .get::<String>("tid")
                    .ok_or(Status::invalid_argument("tid is required"))?;

                wrap_result!(ack, name, executor.act().skip(&pid, &tid, options))
            }
            "act:error" => {
                let pid = options
                    .get::<String>("pid")
                    .ok_or(Status::invalid_argument("pid is required"))?;
                let tid = options
                    .get::<String>("tid")
                    .ok_or(Status::invalid_argument("tid is required"))?;

                wrap_result!(ack, name, executor.act().error(&pid, &tid, options))
            }
            // model
            "model:ls" => {
                let count = options.get::<i64>("count").map_or(100, |v| v as usize);
                let ret = executor.model().list(count);
                wrap_result!(ack, name, ret)
            }
            "model:rm" => {
                let id = options
                    .get::<String>("id")
                    .ok_or(Status::invalid_argument("id is required"))?;
                let ret = executor.model().rm(&id);
                wrap_result!(ack, name, ret)
            }
            "model:get" => {
                let mid = options
                    .get::<String>("id")
                    .ok_or(Status::invalid_argument("id is required"))?;
                let fmt = options.get::<String>("fmt").unwrap_or("text".to_string());
                let ret = executor.model().get(&mid, &fmt);
                wrap_result!(ack, name, ret)
            }
            "model:deploy" => {
                let model_text = options
                    .get::<String>("model")
                    .ok_or(Status::invalid_argument("model is required"))?;

                let mut model =
                    Workflow::from_yml(&model_text).map_err(|err| Status::invalid_argument(err))?;
                if let Some(mid) = options.get::<String>("mid") {
                    model.set_id(&mid);
                };
                wrap_result!(ack, name, executor.model().deploy(&model))
            }
            // package
            "pack:ls" => {
                let count = options.get::<i64>("count").map_or(100, |v| v as usize);
                let ret = executor.pack().list(count);
                wrap_result!(ack, name, ret)
            }
            "pack:publish" => {
                let package_id = options
                    .get::<String>("id")
                    .ok_or(Status::invalid_argument("package 'id' is required"))?;
                let package_name = options.get::<String>("name").unwrap_or_default();
                let data = options
                    .get::<String>("body")
                    .ok_or(Status::invalid_argument("package 'body' is required"))?;
                let pack = Package {
                    id: package_id,
                    name: package_name,
                    data: data.into_bytes(),
                    ..Default::default()
                };
                wrap_result!(ack, name, executor.pack().publish(&pack))
            }
            "pack:rm" => {
                let id = options
                    .get::<String>("id")
                    .ok_or(Status::invalid_argument("id is required"))?;
                let ret = executor.pack().rm(&id);
                wrap_result!(ack, name, ret)
            }
            // proc
            "proc:start" => {
                let id = options
                    .get::<String>("id")
                    .ok_or(Status::invalid_argument("id is required"))?;
                wrap_result!(ack, name, executor.proc().start(&id, options))
            }
            "proc:ls" => {
                let count = options.get::<i64>("count").map_or(100, |v| v as usize);
                let ret = executor.proc().list(count);
                wrap_result!(ack, name, ret)
            }
            "proc:get" => {
                let pid = options
                    .get::<String>("pid")
                    .ok_or(Status::invalid_argument("pid is required"))?;
                let ret = executor.proc().get(&pid);
                wrap_result!(ack, name, ret)
            }
            // task
            "task:ls" => {
                let pid = options
                    .get::<String>("pid")
                    .ok_or(Status::invalid_argument("pid is required"))?;
                let count = options.get::<i64>("count").map_or(100, |v| v as usize);
                let ret = executor.task().list(&pid, count);
                wrap_result!(ack, name, ret)
            }
            "task:get" => {
                let pid = options
                    .get::<String>("pid")
                    .ok_or(Status::invalid_argument("pid is required"))?;
                let tid = options
                    .get::<String>("tid")
                    .ok_or(Status::invalid_argument("tid is required"))?;
                let ret = executor.task().get(&pid, &tid);
                wrap_result!(ack, name, ret)
            }
            // msg
            "msg:ls" => {
                let pid = options
                    .get::<String>("pid")
                    .ok_or(Status::invalid_argument("pid is required"))?;
                let count = options.get::<i64>("count").map_or(100, |v| v as usize);
                let ret = executor.msg().list(&pid, count);
                wrap_result!(ack, name, ret)
            }
            "msg:get" => {
                let id = options
                    .get::<String>("id")
                    .ok_or(Status::invalid_argument("id is required"))?;
                let ret = executor.msg().get(&id);
                wrap_result!(ack, name, ret)
            }
            "msg:ack" => {
                let id = options
                    .get::<String>("id")
                    .ok_or(Status::invalid_argument("id is required"))?;
                wrap_result!(ack, name, executor.msg().ack(&id))
            }
            "msg:redo" => {
                let ret = executor.msg().redo();
                wrap_result!(ack, name, ret)
            }
            "msg:rm" => {
                let id = options
                    .get::<String>("id")
                    .ok_or(Status::invalid_argument("id is required"))?;
                let ret = executor.msg().rm(&id);
                wrap_result!(ack, name, ret)
            }
            _ => Err(Status::not_found(format!("not found action '{name}'"))),
        }
    }
}

#[tonic::async_trait]
impl ActsService for GrpcServer {
    type OnMessageStream = MessageStream;
    async fn send(
        &self,
        request: tonic::Request<Message>,
    ) -> Result<tonic::Response<Message>, tonic::Status> {
        self.do_action(request.into_inner())
    }

    async fn on_message(
        &self,
        request: tonic::Request<MessageOptions>,
    ) -> Result<tonic::Response<Self::OnMessageStream>, tonic::Status> {
        let (tx, rx) = mpsc::channel::<Result<Message, Status>>(128);
        let mut clients = self.clients.lock().await;

        let addr = request.remote_addr().unwrap();
        let options = request.into_inner();

        // tracing::info!("on_message: options={:?}", options);
        if clients.contains_key(&options.client_id) {
            clients.remove(&options.client_id);
        }
        let client = MessageClient {
            addr: addr.to_string(),
            sender: tx,
            options: ChannelOptions {
                r#type: options.r#type.clone(),
                state: options.state.clone(),
                tag: options.tag.clone(),
                key: options.key.clone(),
                ack: true,
                id: options.client_id.clone(),
            },
        };
        clients
            .entry(options.client_id)
            .and_modify(|entry| *entry = client.clone())
            .or_insert(client.clone());

        let chan = self.engine.channel_with_options(&client.options);
        let c = client.clone();
        chan.on_message(move |e| {
            let message = Message {
                name: e.name.clone(),
                seq: e.id.clone(),
                ack: None,
                data: Some(serde_json::to_vec(e.inner()).unwrap()),
            };
            c.send(message);
        });

        let out_stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(out_stream) as Self::OnMessageStream))
    }
}
