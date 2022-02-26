use async_trait::async_trait;

use crate::dicts::{
    baidu::{BaiduDict, BaiduResponse},
    caiyun::{CaiyunDict, CaiyunResponse},
    youdao::{YoudaoDict, YoudaoSimpleResponse},
};

#[async_trait]
pub trait Translator {
    async fn search(&self);
}

#[async_trait]
impl Translator for YoudaoDict {
    async fn search(&self) {
        let headers = (&self.headers).try_into().expect("youdao valid headers");
        let rest = reqwest::Client::new()
            .post(&self.url)
            .headers(headers)
            .form(&self)
            .send()
            .await
            .expect("youdao post request error");

        let response: YoudaoSimpleResponse =
            serde_json::from_str(&rest.text().await.expect("get youdao search error")).unwrap();

        println!("{}", response);
    }
}

#[async_trait]
impl Translator for BaiduDict {
    async fn search(&self) {
        let headers = (&self.headers).try_into().expect("baidu valid headers");
        let rest = reqwest::Client::new()
            .post(&self.url)
            .headers(headers)
            .form(&self)
            .send()
            .await
            .expect("baidu post request error");

        let response: BaiduResponse =
            serde_json::from_str(&rest.text().await.expect("get youdao search error")).unwrap();

        println!("{}", response);
    }
}

#[async_trait]
impl Translator for CaiyunDict {
    async fn search(&self) {
        let headers = (&self.headers).try_into().expect("caiyun valid headers");

        let body = serde_json::to_string(self).unwrap();

        let rest = reqwest::Client::new()
            .post(&self.url)
            .headers(headers)
            .body(body)
            .send()
            .await
            .expect("caiyun request url error");

        let response: CaiyunResponse =
            serde_json::from_str(&rest.text().await.expect("get youdao search error")).unwrap();

        println!("{}", response);
    }
}
