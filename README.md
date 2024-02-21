# acts-channel
[![Build](https://github.com/yaojianpin/acts-channel/actions/workflows/build.yml/badge.svg)](https://github.com/yaojianpin/acts-channel/actions?workflow=build)

provides an acts client channel for workflow engine server [`acts-server`](<https://github.com/yaojianpin/acts-server>)

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

Before connecting, please download  [`acts-server`](<https://github.com/yaojianpin/acts-server>) and start it

## Message
Listening to the message from [`acts-server`](<https://github.com/yaojianpin/acts-server>)

```rust,ignore
use acts_channel::{ActsChannel, ActsOptions};

fn on_message(msg: &Message) {
    // do something
}
let uri = format!("http://{hostname}:{port}");
let client = ActsChannel::new(
    &uri,
    "my_client_id",

    // the ActsOptions can set to filter the messages with type, event, tag and key
    ActsOptions {
        on_message: Some(on_message),
        ..ActsOptions::default()
    },
)
.await

```

## Action

Executes action to interact with acts-server, such as `deploy`, `start`, `push`, `remove`, `submit`, `complete`, `back`, `cancel`, `skip`, `error`, etc. For more information, please see [`acts-server`](<https://github.com/yaojianpin/acts-server>)

### Deploy
```rust,ignore
let resp = client
    .deploy("mid", "model yml here").await?;
let result: ActionResult = resp.into_inner();

```

### Start
```rust,ignore
let mut vars = Vars::new();
vars.insert("var1", &true.into());
let resp = client
    .submit("pid", "tid", &vars).await?;
let result: ActionResult = resp.into_inner();

```

### Complete
```rust,ignore
let mut vars = Vars::new();
vars.insert("var1", json!("value1"));
let resp = client
    .complete("pid", "tid", &vars).await?;
let result: ActionResult = resp.into_inner();

```

### Back
```rust,ignore
let mut vars = Vars::new();
vars.insert("to", &json!("step1"));
let resp = client
    .back("pid", "tid", &vars).await?;
let result: ActionResult = resp.into_inner();

```

### Cancel
```rust,ignore
let mut vars = Vars::new();
vars.insert("var1", json!("value1"));
let resp = client
    .cancel("pid", "tid", &vars).await?;
let result: ActionResult = resp.into_inner();

```

### Skip
```rust,ignore
let mut vars = Vars::new();
vars.insert("var1", json!("value1"));
let resp = client
    .skip("pid", "tid", &vars).await?;
let result: ActionResult = resp.into_inner();

```

### Error
```rust,ignore
let mut vars = Vars::new();
vars.insert("error_code", json!("err1"));
let resp = client
    .error("pid", "tid", &vars).await?;
let result: ActionResult = resp.into_inner();

```

### Push
```rust,ignore
let mut vars = Vars::new();
vars.insert("var1", json!("value1"));
let resp = client
    .push("pid", "tid", &vars).await?;
let result: ActionResult = resp.into_inner();

```

### Remove
```rust,ignore
let mut vars = Vars::new();
vars.insert("var1", json!("value1"));
let resp = client
    .remove("pid", "tid", &vars).await?;
let result: ActionResult = resp.into_inner();

```
