use serde::{Serialize, Deserialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct FeiXiaoHao {
    id: String,
    price_usd: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
            if i.id.eq("bitcoin") {
                s.push(format!("<span font='Font Awesome 5 Free'>&#xf379;</span> {}<span font='Font Awesome 5 Free'>&#xf155;</span>", i.price_usd));
            } else if i.id.eq("ethereum") {
                s.push(format!("<span font='Font Awesome 5 Free'>&#xf42e;</span> {}<span font='Font Awesome 5 Free'>&#xf155;</span>", i.price_usd));
            }
        }
        println!("{}", s.join::<&str>(" "));
        feixiaohao_list.truncate(0);
        s.truncate(0);
        std::thread::sleep(Duration::from_secs(30));
    }
    Ok(())
}
