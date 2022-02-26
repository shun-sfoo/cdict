use std::{collections::HashMap, env};

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CaiyunDict {
    #[serde(skip)]
    pub url: String,
    #[serde(skip)]
    pub headers: HashMap<String, String>,
    source: String,
    trans_type: String,
    request_id: String,
}

impl CaiyunDict {
    pub fn new(content: &str) -> Self {
        let token = env::var("CAIYUN_TOKEN").expect("miss caiyun token");
        let token = "token ".to_owned() + &token;
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        headers.insert("x-authorization".to_string(), token);

        Self {
            url: "http://api.interpreter.caiyunai.com/v1/translator".to_string(),
            headers,
            source: content.to_string(),
            trans_type: "en2zh".to_string(),
            request_id: "demo".to_string(),
        }
    }
}

//{"rc":0,"target":"\u6d4b\u91cf\u4e24\u4e2a\u4ee3\u7801\u6bb5\u4e4b\u95f4\u7684\u8fd0\u884c\u65f6\u95f4","confidence":0.9288572035,"src_tgt":{},"isdict":0}
//
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CaiyunResponse {
    rc: i32,
    target: String,
    confidence: f64,
    // src_tgt unkonw
    #[serde(skip)]
    isdict: i32,
}

impl std::fmt::Display for CaiyunResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        result = result + "caiyun: \n" + &self.target;
        f.write_str(&result)
    }
}
