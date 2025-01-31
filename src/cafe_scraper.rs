use scraper::Html;
use scraper::Selector;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let result = get_reqwest().await?;
    try_parse_html(&result);
    Ok(())
}

async fn get_reqwest() -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    Ok(body)
}

fn try_parse_html(html: &str) {
    let document = Html::parse_document(html);
    let selector_str = "h2";
    let selector = Selector::parse(selector_str).unwrap();

    for element in document.select(&selector) {
        if let Some(unicode) = element.text().next() {
            println!("{}", unicode);
        }
    }
}
