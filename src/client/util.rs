use serde::Serialize;
use serde_json::{to_string, to_value, Value};

pub trait ToUrlQuery: Serialize {
    fn to_url_query(&self) -> Vec<(String, String)> {
        let v = to_value(self).unwrap();
        let v = match v {
            Value::Null => return vec![],
            Value::Object(v) => v,
            _ => panic!("expected struct as url params"),
        };

        let mut vec: Vec<(String, String)> = vec![];

        for (key, value) in v.into_iter() {
            match value {
                Value::Null => continue,
                Value::String(s) => vec.push((key, s)),
                Value::Array(a) => vec.push((
                    key,
                    a.iter()
                        .map(|v| v.as_str().unwrap())
                        .collect::<Vec<_>>()
                        .join(","),
                )),
                v => vec.push((key, to_string(&v).unwrap())),
            }
        }
        vec
    }
}

impl<S: Serialize> ToUrlQuery for S {}

pub trait HeaderBuilder {
    fn add_header(self, key: &str, value: &str) -> Self;
}

use tokio_tungstenite::tungstenite::http::request::Builder as TungsteniteRequestBuilder;

impl HeaderBuilder for TungsteniteRequestBuilder {
    fn add_header(self, key: &str, value: &str) -> Self {
        self.header(key, value)
    }
}

use reqwest::RequestBuilder as ReqwestBuilder;

impl HeaderBuilder for ReqwestBuilder {
    fn add_header(self, key: &str, value: &str) -> Self {
        self.header(key, value)
    }
}
