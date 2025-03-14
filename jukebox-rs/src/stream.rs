use actix_web::{HttpResponse, HttpResponseBuilder, Responder, http::StatusCode, web};
use jukebox_channel::{ChannelCommand, Stream};

pub(crate) async fn api_stream(channel_manager: web::Data<ChannelCommand>) -> impl Responder {
    let mut builder = HttpResponseBuilder::new(StatusCode::OK);
    builder.content_type("audio/mpg");
    let stream = Stream::default();
    channel_manager
        .register("test", &stream)
        .await
        .map(|_| builder.body(stream))
        .unwrap_or(HttpResponse::InternalServerError().finish())
}
