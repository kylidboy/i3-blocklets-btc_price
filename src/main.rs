use serde::{Serialize, Deserialize};
use std::time::Duration;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct FeiXiaoHao {
    id: String,
    price_usd: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let label_btc = env::var("LABEL_BTC");
    let label_eth = env::var("LABEL_ETH");
    let label_usd = env::var("LABEL_USD");

    let headers = http::HeaderMap::new();
    let client = reqwest::Client::builder().timeout(Duration::from_secs(10)).default_headers(headers).build().unwrap();
    let mut feixiaohao_list: Vec<FeiXiaoHao> = vec![];
    let mut s: Vec<String> = vec![];
    loop {
        feixiaohao_list = client.get("https://fxhapi.feixiaohao.com/public/v1/ticker?limit=2").send()
            .await?
        .json()
        .await?;
        for i in feixiaohao_list.iter() {
            if i.id.eq("bitcoin") && label_btc.is_ok() {
                let mut btc = format!("{} {}", label_btc.clone().unwrap(), i.price_usd);
                if label_usd.is_ok() {
                    btc = format!("{}{}", &btc, label_usd.clone().unwrap());
                }
                s.push(btc);
            } else if i.id.eq("ethereum") && label_eth.is_ok() {
                let mut eth = format!("{} {}", label_eth.clone().unwrap(), i.price_usd);
                if label_usd.is_ok() {
                    eth = format!("{}{}", &eth, label_usd.clone().unwrap());
                }
                s.push(eth);
            }
        }
        println!("{}", s.join::<&str>("   "));
        feixiaohao_list.truncate(0);
        s.truncate(0);
        std::thread::sleep(Duration::from_secs(30));
    }
    Ok(())
}
