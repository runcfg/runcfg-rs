#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use super::*;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ExampleConfig {
        #[serde(rename = "version")]
        pub version: String,
        #[serde(rename = "target")]
        pub target: String,
        #[serde(rename = "enabled")]
        pub enabled: String,
    }

    #[test]
    fn client_create() {
        let client = runcfg::Client::new();
        let exampleConfig:  ExampleConfig = client.load().unwrap();
        assert_eq!(exampleConfig.version, "1");
    }
}

pub mod runcfg {
    use serde::{Deserialize, Serialize};
    use std::fs;
    use reqwest;
    use serde::de::DeserializeOwned;

    #[derive(Serialize, Deserialize, Clone)]
    pub struct ClientAuth {
        #[serde(rename = "projectId")]
        pub project_id: String,
        #[serde(rename = "clientToken")]
        client_token: String,
    }

    pub(crate) struct Client {
        pub req: reqwest::blocking::RequestBuilder,
        pub contents: Box<str>,
    }
    impl Client {
        pub fn new() -> Client {
            let conf = fs::read_to_string(".runcfg").expect("[.runcfg] Failed to load .runcfg file");
            let client_conf: ClientAuth = serde_json::from_str(&conf).expect("[.runcfg] Failed to read content of .runcfg file");

            let target = format!("https://runcfg.com/app/project/{}/view", &client_conf.project_id);
            let client = reqwest::blocking::Client::new();
            let builder = client.get(target)
                .header("Authorization", &client_conf.client_token)
                .header("User-Agent", "runcfg-rs/1.0.0");

            Client {
                req: builder,
                contents: Box::from(""),
            }
        }

        pub fn load<'de, T>(mut self) -> Option<T> where T: DeserializeOwned + std::fmt::Debug {
            let resp = self.req.send().unwrap();
            self.contents = Box::from(resp.text().expect("[.runcfg] failed to read config"));
            let cont = self.contents.trim_matches(|x| x == '\"');
            let stripped = unescape(cont).expect("[.runcfg] un-escaping string failed");
            let config = serde_json::from_str::<T>(format!("{}", stripped).as_str());
            match config {
                Ok(a ) => {
                    Some(a)
                },
                Err(_) => {
                    None
                }
            }
        }
    }

    fn unescape(s: &str) -> Option<String> {
        let mut n = String::new();
        let mut ch = s.chars();
        while let Some(c) = ch.next() {
            n.push(match c {
                '\\' => ch.next()?,
                c => c,
            });
        }
        Some(n)
    }
}