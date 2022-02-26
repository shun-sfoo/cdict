use std::{
    collections::HashMap,
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use crypto::{digest::Digest, sha2::Sha256};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct YoudaoDict {
    #[serde(skip)]
    pub url: String,
    #[serde(skip)]
    pub headers: HashMap<String, String>,
    from: String,
    to: String,
    curtime: String,
    #[serde(rename(serialize = "appKey"))]
    app_key: String,
    q: String,
    salt: String,
    sign: String,
    #[serde(rename(serialize = "signType"))]
    sign_type: String,
    #[serde(rename(serialize = "vocabId"))]
    vocab_id: String,
}

impl YoudaoDict {
    pub fn new(content: &str) -> Self {
        let curtime = SystemTime::now();
        let duration = curtime
            .duration_since(UNIX_EPOCH)
            .expect("timestap covert duration error");
        let duration = duration.as_secs();
        let curtime = duration.to_string();
        let salt = Uuid::new_v4().to_string();

        let len = content.len();

        let input = if len > 20 {
            let head = (&content[..10]).to_string();
            let tail = (&content[len - 10..]).to_string();
            head + &len.to_string() + &tail
        } else {
            (&content).to_string()
        };

        let app_key = env::var("YOUDAO_APP_KEY").expect("miss youdao app_key");
        let app_secert = env::var("YOUDAO_SECERT").expect("miss youdao app_secert");

        let sign_str = (&app_key).to_string() + &input + &salt + &curtime + &app_secert;

        let mut hasher = Sha256::new();
        hasher.input_str(&sign_str);
        let sign = hasher.result_str();

        let mut headers = HashMap::new();

        headers.insert(
            "content_type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        Self {
            url: "https://openapi.youdao.com/api".to_string(),
            headers,
            q: content.to_string(),
            from: "auto".into(),
            to: "auto".into(),
            app_key,
            salt,
            sign,
            sign_type: "v3".to_string(),
            curtime,
            vocab_id: "1".to_string(),
        }
    }
}

//{"tSpeakUrl":"https://openapi.youdao.com/ttsapi?q=%E6%B5%8B%E9%87%8F%E4%B8%A4%E4%B8%AA%E4%BB%A3%E7%A0%81%E6%AE%B5%E4%B9%8B%E9%97%B4%E7%9A%84%E8%BF%90%E8%A1%8C%E6%97%B6%E9%97%B4&langType=zh-CHS&sign=85700976A24FD20849C8CAF72BDE4A40&salt=1645862414204&voice=4&format=mp3&appKey=263d42fdda3ed4e3&ttsVoiceStrict=false","requestId":"377714f7-e03a-43c1-a088-93fe0e297df7","query":"Measure the elapsed time between two code sections","translation":["测量两个代码段之间的运行时间"],"errorCode":"0","dict":{"url":"yddict://m.youdao.com/dict?le=eng&q=Measure+the+elapsed+time+between+two+code+sections"},"webdict":{"url":"http://mobile.youdao.com/dict?le=eng&q=Measure+the+elapsed+time+between+two+code+sections"},"l":"en2zh-CHS","isWord":false,"speakUrl":"https://openapi.youdao.com/ttsapi?q=Measure+the+elapsed+time+between+two+code+sections&langType=en&sign=6E5032DF894141BA756F4A99AC6E89F6&salt=1645862414204&voice=4&format=mp3&appKey=263d42fdda3ed4e3&ttsVoiceStrict=false"}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct YoudaoSimpleResponse {
    pub translation: Vec<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct YoudaoResponse {
    #[serde(rename(deserialize = "tSpeakUrl"))]
    t_speak_url: String,
    #[serde(rename(deserialize = "requestId"))]
    rquest_id: String,
    query: String,
    translation: Vec<String>,
    #[serde(rename(deserialize = "errorCode"))]
    error_code: String,
    dict: Dict, // struct
    webdict: Dict,
    l: String,
    #[serde(rename(deserialize = "isWord"))]
    is_word: bool,
    #[serde(rename(deserialize = "speakUrl"))]
    speak_url: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Dict {
    url: String,
}

impl std::fmt::Display for YoudaoSimpleResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        result = result + "youdao: \n";
        for i in 0..self.translation.len() {
            if i != self.translation.len() - 1 {
                result = result + &self.translation[i] + "\n";
            } else {
                result = result + &self.translation[i];
            }
        }
        f.write_str(&result)
    }
}
