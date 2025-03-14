use actix_web::{HttpResponse, HttpResponseBuilder, Responder, http::StatusCode, web};
use jukebox_channel::ChannelCommand;

pub(crate) async fn api_next(channel_manager: web::Data<ChannelCommand>) -> impl Responder {
    let mut builder = HttpResponseBuilder::new(StatusCode::NO_CONTENT);
    channel_manager
        .next("test")
        .await
        .map(|_| builder.finish())
        .unwrap_or(HttpResponse::InternalServerError().finish())
}

pub(crate) async fn api_previous(channel_manager: web::Data<ChannelCommand>) -> impl Responder {
    let mut builder = HttpResponseBuilder::new(StatusCode::NO_CONTENT);
    builder.content_type("audio/mpg");
    channel_manager
        .previous("test")
        .await
        .map(|_| builder.finish())
        .unwrap_or(HttpResponse::InternalServerError().finish())
}
