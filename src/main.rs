use serde::{Serialize, Deserialize};
use std::time::Duration;
use std::env;

const COINBASE_BTC_URL: &str = "https://api.pro.coinbase.com/products/BTC-USD/ticker";
const COINBASE_ETH_URL: &str = "https://api.pro.coinbase.com/products/ETH-USD/ticker";
const FEIXIAOHAO_URL: &str = "https://fxhapi.feixiaohao.com/public/v1/ticker?limit=2";

#[derive(Serialize, Deserialize, Debug)]
struct FeiXiaoHao {
    id: String,
    price_usd: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct CoinbasePro {
    price: String,
}
impl Default for CoinbasePro {
    fn default() -> Self {
        CoinbasePro{ price: String::new() }
    }
}

type EnvVar = Result<String, env::VarError>;

#[derive(Copy, Clone)]
enum Source {
    Coinbase,
    Feixiaohao,
}
impl Source {
    pub fn from_result(r: Result<String, impl std::error::Error>) -> Self {
        if r.is_ok() && r.unwrap().eq("feixiaohao") {
            Source::Feixiaohao
        } else {
            Source::Coinbase
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = env::var("CRYPTO_SOURCE");
    let label_btc = env::var("LABEL_BTC");
    let label_eth = env::var("LABEL_ETH");
    let label_usd = env::var("LABEL_USD");
    let mut headers = http::HeaderMap::new();
    headers.insert("Accpet", http::HeaderValue::from_str("application/json").unwrap());
    let client = reqwest::Client::builder().timeout(Duration::from_secs(30)).default_headers(headers).build().unwrap();
    let source = Source::from_result(source);
    loop {
        let r = match source {
            Source::Coinbase => {
                coinbasepro(&client, (&label_btc, &label_eth, &label_usd)).await
            }
            Source::Feixiaohao => {
                feixiaohao(&client, (&label_btc, &label_eth, &label_usd)).await
            }
        };
        match r {
            Ok(s) => {
                println!("{}", s);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }

        std::thread::sleep(Duration::from_secs(30));
    }
}

async fn coinbasepro(client: &reqwest::Client, (label_btc, label_eth, label_usd): (&EnvVar, &EnvVar, &EnvVar)) -> Result<String, Box<dyn std::error::Error>> {
    let mut res: CoinbasePro = Default::default();
    let mut s: Vec<String> = vec![];

    if label_btc.is_ok() {
        res = client.get(COINBASE_BTC_URL).send().await?.json().await?;
        let mut btc = format!("{} {}", label_btc.clone().unwrap(), res.price);
        if label_usd.is_ok() {
            btc = format!("{}{}", &btc, label_usd.clone().unwrap());
        }
        s.push(btc);
    }
    if label_eth.is_ok() {
        res = client.get(COINBASE_ETH_URL).send().await?.json().await?;
        let mut eth = format!("{} {}", label_eth.clone().unwrap(), res.price);
        if label_usd.is_ok() {
            eth = format!("{}{}", &eth, label_usd.clone().unwrap());
        }
        s.push(eth);
    }
    Ok(s.join::<&str>("   "))
}

async fn feixiaohao(client: &reqwest::Client, (label_btc, label_eth, label_usd): (&EnvVar, &EnvVar, &EnvVar)) -> Result<String, Box<dyn std::error::Error>> {
    let mut list: Vec<FeiXiaoHao> = vec![];
    let mut s: Vec<String> = vec![];
    list = client.get(FEIXIAOHAO_URL).send()
                                     .await?
        .json()
        .await?;
        for i in list.iter() {
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
    Ok(s.join::<&str>("   "))
}
