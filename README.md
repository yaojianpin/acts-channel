# acts-channel
provides an acts client channel for acts-server

The crate is called acts-channel and you can depend on it via cargo:
```toml
[dependencies]
acts-channel = "*"
```
If you want to use the git version:
```toml
[dependencies]
acts-channel = { git = "https://github.com/yaojianpin/acts-channel.git" }
```

# Usage

## Message
Listening to the message from [`acts-server`](<https://github.com/yaojianpin/acts-server>)

```rust,no_run
use acts_channel::{
    self, acts_service_client::ActsServiceClient, model::Message, ActionOptions, MessageOptions,
    Vars,
};
use tonic::{transport::Channel, Request, Status};

let uri = format!("http://{hostname}:{port}");
let endpoint = Endpoint::from_str(&uri)?;
let mut client = ActsServiceClient::connect(endpoint).await?;

let request = tonic::Request::new(MessageOptions {
    client_id: "my_client_id".to_string(),
    r#type: "*".to_string(),
    state: "*".to_string(),
    tag: "*".to_string(),
});
let mut stream = client.on_message(request).await.unwrap().into_inner();
tokio::spawn(async move {
    while let Some(item) = stream.next().await {
        if !item.is_err() {
            let m: Message = item.unwrap().into();
            println!("[message]: {}", serde_json::to_string(&m).unwrap());
        }
    }
});

```

## Action

Executes action to interact with acts-server, such as `deploy`, `start`, `submit`, `complete`, `back`, `cancel`, `error`, etc. For more information, please see [`acts-server`](<https://github.com/yaojianpin/acts-server>)

### Deploy
```rust,no_run

let mut options = Vars::new();
options.insert_str("model".to_string(), "model yml here");

let resp = client
    .action(Request::new(ActionOptions {
        // action name
        name: "deploy".to_string(),
        options: Some(options.prost_vars()),
    }))
    .await?;
let result: ActionResult = resp.into_inner();

```


### Start
```rust,no_run
let mut options = Vars::new();
options.insert_str("mid".to_string(), "your_model_id".to_string());
options.insert_str("pid".to_string(), "your_custom_id_for_proc_id".to_string());

let resp = self
    .client
    .action(Request::new(ActionOptions {
        name: "start".to_string(),
        options: Some(options.prost_vars()),
    }))
    .await?;
let result: ActionResult = resp.into_inner();

```
