# bevy_ios_alerts

[![crates.io](https://img.shields.io/crates/v/bevy_ios_alerts.svg)](https://crates.io/crates/bevy_ios_alerts)

Rust crate and Swift package to easily integrate iOS's native UIAlerts API into a Bevy application.

![demo](./assets/demo.gif)

demo from our game using this crate: [zoolitaire.com](https://zoolitaire.com)

## Instructions

1. Add to XCode: Add SPM (Swift Package Manager) dependency
2. Add Rust dependency
3. Setup Plugin

### 1. Add to XCode

Go to `File` -> `Add Package Dependencies` and paste `https://github.com/rustunit/bevy_ios_alerts.git` into the search bar on the top right:
![xcode](./assets/xcode-spm.png)

### 2. Add Rust dependency

```
cargo add bevy_ios_alerts
``` 

or 

```
bevy_ios_alerts = { version = "0.1" }
```

### 3. Setup Plugin

Initialize Bevy Plugin:

```rust
app.add_plugins(bevy_ios_alerts::IosAlertsPlugin);
```

Trigger Alert in your application code:

```rust
fn system_triggerin_alerts(mut events: EventWriter<IosAlert>) {
     
    events.send(IosAlert::Message {
        title: String::from("title"),
        msg: String::from("msg"),
        button: String::from("ok"),
    });
     
    events.send(IosAlert::Dialog {
        title: String::from("title"),
        msg: String::from("this is a dialog with multiple buttons"),
        button_yes: String::from("absolutely yes"),
        button_no: String::from("no no no"),
    });
                    
    events.send(IosAlert::Input {
        title: String::from("title"),
        msg: String::from("input box"),
        button: String::from("ok"),
        placeholder: String::from("placeholder"),
    });
}

fn process_alert_response(mut events: EventReader<IosAlertResponse>) {
    for e in events.read() {
        info!("incoming alert response: {e:?}");
    }
}
```