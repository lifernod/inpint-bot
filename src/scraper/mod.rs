use reqwest::Client;
use scraper::Html;

pub(crate) mod pin_img;

pub(self) async fn load_html(url: &String) -> Result<Html, reqwest::Error> {
    let client = Client::new();

    let res = client.get(url).send().await?;
    Ok(Html::parse_document(res.text().await?.as_str()))
}
