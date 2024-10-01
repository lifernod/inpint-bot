use reqwest::Url;
use teloxide::{
    prelude::*,
    types::{
        InlineQueryResult, InlineQueryResultArticle, InlineQueryResultPhoto, InputMessageContent,
        InputMessageContentText,
    },
    RequestError,
};
use uuid::Uuid;

use crate::scraper::pin_img::get_pin_image;

pub async fn handle_pin_url(bot: Bot, query: InlineQuery) -> Result<(), RequestError> {
    let pin_url = query.query.replace("pin_img", "").trim().to_string();
    let pin_image = get_pin_image(&pin_url).await;

    let id = Uuid::new_v4().to_string();

    let result = match pin_image {
        Ok(image_url) => {
            let photo_url = Url::parse(&image_url).unwrap();
            InlineQueryResult::Photo(InlineQueryResultPhoto::new(
                &id,
                photo_url.clone(),
                photo_url,
            ))
        }
        Err(_) => InlineQueryResult::Article(InlineQueryResultArticle::new(
            &id,
            "Упс...",
            InputMessageContent::Text(InputMessageContentText::new(format!(
                "Ссылка '{pin_url}' не найдена!"
            ))),
        )),
    };

    let _ = bot.answer_inline_query(&query.id, vec![result]).await;

    Ok(())
}

pub async fn handle_empty_command(bot: Bot, query: InlineQuery) -> Result<(), RequestError> {
    let result = InlineQueryResult::Article(InlineQueryResultArticle::new(
        "01".to_string(),
        "Введите команду...",
        InputMessageContent::Text(InputMessageContentText::new(
            r#"🤷‍♂️ Не введена команда!
        Доступные команды:
            pin_img {pin_url} - Отправка картинки, которая находится по ссылке пина"#,
        )),
    ));

    bot.answer_inline_query(&query.id, vec![result])
        .send()
        .await?;

    Ok(())
}
