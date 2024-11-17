# acts-channel

[![Build](https://github.com/yaojianpin/acts-channel/actions/workflows/build.yml/badge.svg)](https://github.com/yaojianpin/acts-channel/actions?workflow=build)

provides an acts client channel for workflow engine server [`acts-server`](https://github.com/yaojianpin/acts-server)

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

Before connecting, please download [`acts-server`](https://github.com/yaojianpin/acts-server) and start it

## Message

Listening to the message from [`acts-server`](https://github.com/yaojianpin/acts-server)

```rust,ignore
use acts_channel::{ActsChannel, ActsOptions};

let url = format!("http://{hostname}:{port}");

// connect to server
let mut client = ActsChannel::connect(&url).await?;

// subscribe the messages
client
    .subscribe(
        "client-1",
        move |message| {
            println!("{message:?}");
        },
        &ActsOptions::default(),
    )
    .await;

```

## Action

Executes action to interact with acts-server, such as `publish`, `start`, `push`, `remove`, `submit`, `complete`, `back`, `cancel`, `skip`, `error`, etc. For more information, please see [`acts-server`](https://github.com/yaojianpin/acts-server)

### Publish

```rust,ignore
let yml = r"
    id: test
    name: model test
    steps:
        - name: step 1
    ";
let resp = client
    .publish(yml, Some("custom_model_id")).await?;
assert_eq!(resp.data.unwrap(), true);
```

### Start

```rust,ignore
let mut vars = Vars::new();
vars.set("var1", true);
client
    .submit("pid", "tid", vars).await?;

```

### Complete

```rust,ignore
let mut vars = Vars::new();
vars.set("var1", "value1");
client
    .complete("pid", "tid", vars).await?;
```

### Back

```rust,ignore
let mut vars = Vars::new();
vars.set("to", "step1");
client
    .back("pid", "tid", vars).await?;
```

### Cancel

```rust,ignore
let mut vars = Vars::new();
vars.set("var1", json!("value1"));
client
    .cancel("pid", "tid", vars).await?;

```

### Skip

```rust,ignore
let mut vars = Vars::new();
vars.set("var1", "value1");
client
    .skip("pid", "tid", vars).await?;
```

### Error

```rust,ignore
let mut vars = Vars::new();
vars.set("error", Vars::new().with("ecode", "err1"));
let resp = client
    .error("pid", "tid", vars).await?;
```

### Push

```rust,ignore
let mut vars = Vars::new();
vars.set("var1", "value1");
let resp = client
    .push("pid", "tid", vars).await?;
```

### Remove

```rust,ignore
let mut vars = Vars::new();
vars.set("var1", "value1");
let resp = client
    .remove("pid", "tid", vars).await?;
```
