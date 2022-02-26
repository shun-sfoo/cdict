use clap::Parser;

use crate::{
    dicts::{baidu::BaiduDict, caiyun::CaiyunDict, youdao::YoudaoDict},
    translator::Translator,
};

mod dicts;
mod translator;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    r#type: Option<String>,
    #[clap(short, long)]
    content: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    let args = Args::parse();
    let content = args.content;
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
