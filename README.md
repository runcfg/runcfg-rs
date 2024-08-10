# Runcfg Rust Client

### Usage in projects

First download dependency using cargo
```shell
$ cargo add runcfg-rs
```

Alternatively you can add this to your `cargo.toml`
```toml
runcfg = "1.0.0"
```

### Using your first config

1. Create an account at https://runcfg.com
2. Download your .runcfg file from your project page at https://runcfg.com
3. Place your .runcfg file at the root of your project
4. Create an instance of the client in your code as follows:

```rust
use runcfg;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExampleConfig {
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "target")]
    pub target: String,
    #[serde(rename = "enabled")]
    pub enabled: String,
}

fn load_config() {
    let client = runcfg::Client::new();
    let exampleConfig:  ExampleConfig = client.load().unwrap();
    assert_eq!(exampleConfig.version, "1");
}
```

Now your remote config is available in your specified type.

