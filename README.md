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

Executes action to interact with acts-server, such as `deploy`, `start`, `ack`, `send`, etc. For more information, please see [`acts-server`](https://github.com/yaojianpin/acts-server)

### Publish

```rust,ignore
let yml = r"
    id: test
    name: model test
    steps:
        - name: step 1
    ";
let resp = client
    .deploy(yml, Some("custom_model_id")).await?;
assert_eq!(resp.data.unwrap(), true);
```

### Start

```rust,ignore
let mut vars = Vars::new();
vars.set("var1", true);
client
    .submit("pid", "tid", vars).await?;

```

### do act

```rust,ignore
    // set some other vars
    let vars = Vars::new();

    // combine with pid and tid
    let options = Vars::new().with("pid", pid).with("tid", tid).extend(&vars);

    // name should be one of complete, submit, back, cancel, error, abort, push and remove
    let name = "complete";
    client
        .send::<()>(name, options)
        .await
        .map_err(|err| err.message().to_string())?;
```

### Ack a message

```rust,ignore
    let resp = client
        // id is message id
        .ack(id)
        .await
        .map_err(|err| err.message().to_string())?;
```
