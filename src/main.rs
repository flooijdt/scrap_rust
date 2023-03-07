use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    let url: &str = "https://roadmap.sh/blockchain";
    let req = reqwest::get(url).await.unwrap().text();

    let body = Html::parse_document(&req.await.unwrap());
    let matter = Selector::parse("h2").unwrap();

    for m in body.select(&matter) {
        let matters = m.text().collect::<Vec<_>>();
        println!("{:#?}", &matters[0]);
    }
}
