use std::env::args;

use crate::{
    dicts::{baidu::BaiduDict, caiyun::CaiyunDict, youdao::YoudaoDict},
    translator::Translator,
};

mod dicts;
mod translator;

#[tokio::main]
async fn main() {
    let args: Vec<String> = args().into_iter().collect();
    let content = &args[1];
    let youdao = YoudaoDict::new(&content);
    let caiyun = CaiyunDict::new(&content);
    let baidu = BaiduDict::new(&content);

    let r1 = tokio::spawn(async move { youdao.search().await });
    let r2 = tokio::spawn(async move { caiyun.search().await });
    let r3 = tokio::spawn(async move { baidu.search().await });

    r1.await.unwrap();
    r2.await.unwrap();
    r3.await.unwrap();
}
