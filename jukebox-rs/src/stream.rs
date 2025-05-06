use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder, Responder, http::StatusCode, web};
use jukebox_channel::{ChannelCommand, Stream};

pub(crate) async fn api_stream(
    request: HttpRequest,
    channel_manager: web::Data<ChannelCommand>,
) -> impl Responder {
    let mut builder = HttpResponseBuilder::new(StatusCode::OK);
    builder.content_type("audio/mpeg");
    let stream = Stream::default();
    channel_manager
        .register("test", &stream)
        .await
        .map(|_| {
            let mut res = builder.streaming(stream);
            if request.version() < actix_web::http::Version::HTTP_11 {
                // Disable chunking transfert encoding for HTTP/1.0
                res.head_mut().no_chunking(true);
            }
            res
        })
        .unwrap_or(HttpResponse::InternalServerError().finish())
}
