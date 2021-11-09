use crate::framework::response::ApiResult;
use crate::framework::endpoint::{Endpoint, Method};

/// Read Key Value
/// Returns the value in workers kv
/// https://api.cloudflare.com/#workers-kv-namespace-read-key-value-pair
#[derive(Debug)]
pub struct Read<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub key: &'a str,
}

type Value = String;
impl ApiResult for Value {}

impl<'a> Endpoint<Value> for Read<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("accounts/{}/storage/kv/namespaces/{}/values/{}", self.account_identifier, self.namespace_identifier, self.key)
    }
}
