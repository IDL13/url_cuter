use crate::{app::query::get_full_url::*, dependency_injection::*, storage::*, id_provider::*};
use std::sync::Arc;
use actix_web::{dev::Path,
    get,
    post,
    web::{self, Json, Data},
    App,
    HttpResponse,
    HttpServer,
    Responder,
    Result,
};
use serde::{Deserialize, Serialize};

pub struct Server<I, R, Q>
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortUrlRepository + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    port: u16,
    container: Arc<Container<I, R, Q>>
}

impl<I, R, Q> Server<I, R, Q>
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortUrlRepository + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    pub fn new(port: u16, container: Arc<Container<I, R, Q>>) -> Self {
        Self {
            port,
            container
        }
    }

    pub async fn run(self) -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .route(
                    "/",
                    web::post()
                        .to(shorten_url::<I, R, Q>))
                .route(
                    "/{id}",
                    web::get()
                        .to(get_full_url::<I, R, Q>))
        })
        .bind(("127.0.0.1", 8585))
        .expect("DONT CREATING SERVER")
        .run()
        .await
    }
}

#[derive(Serialize, Deserialize)]
struct CreateShortUrlRequest {
    url: String
}

#[derive(Serialize, Deserialize)]
struct ShortUrlResponse {
    url: String
}

async fn shorten_url<I, R, Q>(
    _container: Data<Arc<Container<I, R, Q>>>,
    Json(_input): Json<CreateShortUrlRequest>,
) -> Result<impl Responder>
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortUrlRepository + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    let resp = _container
    .command
    .execute(_input.url)
    .await
    .map(|id| web::Json(ShortUrlResponse { url:id }))
    .expect("Dummy Error");
    
    Ok(resp)
}

async fn get_full_url<I, R, Q>(
    _container: Data<Arc<Container<I, R, Q>>>,
    Json(_input): Json<CreateShortUrlRequest>,
) -> Result<impl Responder>
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortUrlRepository + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    let resp = _container
    .command
    .execute(_input.url)
    .await
    .map(|id| web::Json(ShortUrlResponse { url:id }))
    .expect("Dummy Error");
    
    Ok(resp)
}