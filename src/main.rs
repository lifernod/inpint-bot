use axum::Router;
use handlers::pin_img::{handle_empty_command, handle_pin_url};
use teloxide::{
    dispatching::{DefaultKey, UpdateHandler},
    prelude::*,
    RequestError,
};

mod handlers;
mod scraper;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = build_router();

    log::info!("Starting bot...");
    Ok(router.into())
}

fn build_router() -> Router {
    let bot = Bot::from_env();
    let mut dp = build_dispatcher(bot);

    tokio::spawn(async move {
        dp.dispatch().await;
    });

    Router::new()
}

fn build_dispatcher(bot: Bot) -> Dispatcher<Bot, RequestError, DefaultKey> {
    Dispatcher::builder(bot, build_handlers())
        .error_handler(LoggingErrorHandler::with_custom_text(
            "Непредвиденная ошибка бота... Попробуйте позже!",
        ))
        .enable_ctrlc_handler()
        .build()
}

fn build_handlers() -> UpdateHandler<RequestError> {
    dptree::entry()
        .branch(
            Update::filter_inline_query()
                .filter(|query: InlineQuery| {
                    !query.query.is_empty() && query.query.contains("pin_img")
                })
                .endpoint(handle_pin_url),
        )
        .branch(
            Update::filter_inline_query()
                .filter(|query: InlineQuery| query.query.is_empty())
                .endpoint(handle_empty_command),
        )
}
