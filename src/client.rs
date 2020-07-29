use chrono::Utc;
use failure::Fallible;
use hmac::{Hmac, Mac, NewMac};
use reqwest::{Client, Method, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{from_str, to_string, to_value, Value};
use sha2::Sha256;
use url::Url;
use log::debug;

pub mod request;

use request::Request;

const HTTP_URL: &'static str = "https://bitmax.io";
const API_URL: &'static str = "/api/pro/v1";

#[derive(Debug, Clone)]
struct Auth {
    pub public_key: String,
    pub private_key_bytes: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct BitMaxClient {
    client: Client,
    auth: Option<Auth>,
}

#[derive(Deserialize, Debug)]
struct ResponseSchema<T> {
    code: u32,
    data: T,
}

impl BitMaxClient {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_auth(public_key: &str, private_key: &str) -> Fallible<Self> {
        Ok(Self {
            auth: Some(Auth {
                private_key_bytes: base64::decode(private_key)?,
                public_key: public_key.into(),
            }),
            client: Default::default(),
        })
    }

    fn attach_auth_headers(
        &self,
        builder: RequestBuilder,
        api_path: &str,
    ) -> Fallible<RequestBuilder> {
        let auth = self
            .auth
            .as_ref()
            .ok_or_else(|| failure::format_err!("Missing auth key"))?;

        let timestamp = Utc::now().timestamp_millis();

        let prehash = format!("{}+{}", timestamp, &api_path[1..]); // skip the first `/`
        debug!("timestamp: {}", &prehash);
        let mut mac = Hmac::<Sha256>::new_varkey(&auth.private_key_bytes)
            .map_err(|e| failure::format_err!("{}", e))?;
        mac.update(prehash.as_bytes());
        let signature = base64::encode(mac.finalize().into_bytes());

        Ok(builder
            .header("x-auth-key", &auth.public_key)
            .header("x-auth-timestamp", timestamp.to_string())
            .header("x-auth-signature", &signature))
    }

    pub async fn request<Q: Request>(&self, request: Q) -> Fallible<Q::Response> {
        let url = if Q::NEEDS_ACCOUNT_GROUP {
            format!("{}/6{}{}", HTTP_URL, API_URL, request.render_endpoint())
        } else {
            format!("{}{}{}", HTTP_URL, API_URL, request.render_endpoint())
        };
        let url = Url::parse_with_params(&url, request.to_url_query())?;

        let req = self
            .client
            .request(Q::METHOD, url.as_str())
            .header("user-agent", "bitmax-rs");

        let req = if Q::NEEDS_AUTH {
            self.attach_auth_headers(req, Q::API_PATH)?
        } else {
            req
        };

        self.handle_response(req.send().await?).await
    }

    async fn handle_response<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        resp: Response,
    ) -> Fallible<T> {
        if resp.status().is_success() {
            let resp = resp.text().await?;
            debug!("got message: {}", &resp);
            match from_str::<ResponseSchema<T>>(&resp) {
                Ok(resp) => {
                    if resp.code == 0 {
                        Ok(resp.data)
                    } else {
                        Err(failure::format_err!("Non zero response code: {:?}", resp))
                    }
                }
                Err(e) => Err(e.into()),
            }
        } else {
            let resp_e = resp.error_for_status_ref().unwrap_err();
            Err(resp_e.into())
        }
    }
}

trait ToUrlQuery: Serialize {
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
