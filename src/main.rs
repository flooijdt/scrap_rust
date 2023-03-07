use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    let url: &str = "https://roadmap.sh/blockchain";
    let req = reqwest::get(url).await.unwrap().text();
    // println!("{:?}", &req.unwrap().text_with_charset("utf-8"));

    let body = Html::parse_document(&req.await.unwrap());
    let matter = Selector::parse(".clickable-group ").unwrap();

    for m in body.select(&matter) {
        let matters = m.text().collect::<Vec<_>>();
        println!("{:#?}", &matters[0]);
    }
}
