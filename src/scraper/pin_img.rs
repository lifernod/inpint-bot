use scraper::{Selector, ElementRef};

use super::load_html;

pub async fn get_pin_image(pin_url: &String) -> Result<String, String> {
    let image_selector = match Selector::parse("img.hCL.kVc.L4E.MIw") {
        Ok(it) => it,
        Err(e) => {
            eprintln!("get_pin_image:image_selector error: {:#?}", e);
            return Err("Opps... Something went wrong, please check pin_url".to_string());
        }
    };
    
    let html = match load_html(pin_url).await {
        Ok(it) => it,
        Err(e) => {
            eprintln!("get_pin_image:html error: {:#?}", e);
            return Err("Opps... Something went wrong, please check pin_url".to_string());
        }
    };

    let images = html.select(&image_selector).collect::<Vec<ElementRef>>();
    let image_url = match images.first() {
        Some(it) => it.attr("src").unwrap(),
        None => {
            eprintln!("get_pin_image:image_url error: images not found (get None)");
            return Err("Opps... Something went wrong, please check pin_url".to_string());
        }
    };


    Ok(String::from(image_url))
}