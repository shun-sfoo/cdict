use std::{
    collections::HashMap,
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use crypto::{digest::Digest, md5::Md5};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct BaiduDict {
    #[serde(skip)]
    pub url: String,
    #[serde(skip)]
    pub headers: HashMap<String, String>,
    appid: String,
    q: String,
    from: String,
    to: String,
    salt: String,
    sign: String,
}

impl BaiduDict {
    pub fn new(content: &str) -> Self {
        let appid = env::var("BAIDU_APP_ID").expect("miss baidu appid");
        let app_secret = env::var("BAIDU_SECERT").expect("miss baidu secret");

        let curtime = SystemTime::now();
        let duration = curtime
            .duration_since(UNIX_EPOCH)
            .expect("timestap covert duration error");

        let duration = duration.as_secs();
        let salt = duration.to_string();

        let sign_str = (&appid).to_string() + &content + &salt + &app_secret;
        let mut hasher = Md5::new();
        hasher.input_str(&sign_str);
        let sign = hasher.result_str();

        let mut headers = HashMap::new();
        headers.insert(
            "content_type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        Self {
            url: "http://api.fanyi.baidu.com/api/trans/vip/translate".to_string(),
            q: (&content).to_string(),
            from: "en".to_string(),
            to: "zh".to_string(),
            headers,
            salt,
            appid,
            sign,
        }
    }
}

//
//{"from":"en","to":"zh","trans_result":[{"src":"Measure the elapsed time between two code sections","dst":"\u6d4b\u91cf\u4e24\u4e2a\u4ee3\u7801\u6bb5\u4e4b\u95f4\u7684\u8fd0\u884c\u65f6\u95f4"}]}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct BaiduResponse {
    from: String,
    to: String,
    trans_result: Vec<TR>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct TR {
    src: String,
    dst: String,
}

impl std::fmt::Display for BaiduResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = &self.trans_result;
        let mut result = String::new();
        result = result + "baidu:\n";

        for i in 0..list.len() {
            if i != list.len() - 1 {
                result = result + &list[i].dst + "\n";
            } else {
                result = result + &list[i].dst;
            }
        }

        f.write_str(&result)
    }
}
