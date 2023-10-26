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

Executes action to interact with acts-server, such as `deploy`, `start`, `submit`, `complete`, `back`, `cancel`, `skip`, `error`, etc. For more information, please see [`acts-server`](<https://github.com/yaojianpin/acts-server>)

### Deploy
```rust,no_run
let resp = client
    .deploy("mid", "model yml here").await?;
let result: ActionResult = resp.into_inner();

```

### Start
```rust,no_run
let mut vars = Vars::new();
vars.insert("var1", &true.into());
let resp = client
    .submit("pid", "tid", "u1", &vars).await?;
let result: ActionResult = resp.into_inner();

```

### Complete
```rust,no_run
let mut vars = Vars::new();
vars.insert("var1", json!("value1"));
let resp = client
    .complete("pid", "tid", "u1", &vars).await?;
let result: ActionResult = resp.into_inner();

```

### Back
```rust,no_run
let mut vars = Vars::new();
vars.insert("to", &json!("step1"));
let resp = client
    .back("pid", "tid", "u1", &vars).await?;
let result: ActionResult = resp.into_inner();

```

### Cancel
```rust,no_run
let mut vars = Vars::new();
vars.insert("var1", json!("value1"));
let resp = client
    .cancel("pid", "tid", "u1", &vars).await?;
let result: ActionResult = resp.into_inner();

```

### Skip
```rust,no_run
let mut vars = Vars::new();
vars.insert("var1", json!("value1"));
let resp = client
    .skip("pid", "tid", "u1", &vars).await?;
let result: ActionResult = resp.into_inner();

```

### Error
```rust,no_run
let mut vars = Vars::new();
vars.insert("error_code", json!("err1"));
let resp = client
    .error("pid", "tid", "u1", &vars).await?;
let result: ActionResult = resp.into_inner();

```
