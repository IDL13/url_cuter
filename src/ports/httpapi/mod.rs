use crate::{app::query::get_full_url::*, dependency_injection::*, storage::*, id_provider::*};
use std::{collections::HashMap, sync::Arc};
use actix_web::{dev::{AppService, Path},
    get,
    post,
    web::{self, resource, Data, Json},
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
                .configure(config_shorten_url::<I, R, Q>)
                .configure(config_get_full_url::<I, R, Q>)
        })
        .bind(("127.0.0.1", 8585))
        .expect("DONT CREATING SERVER")
        .run()
        .await
    }
}

fn config_shorten_url<I, R, Q>(cfg: &mut web::ServiceConfig)
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortUrlRepository + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    cfg.service(
        web::resource("/")
        .route(web::post().to(shorten_url::<I, R, Q>))
    );
}

fn config_get_full_url<I, R, Q>(cfg: &mut web::ServiceConfig)
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortUrlRepository + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    cfg.service(
        web::resource("/{id}")
        .route(web::get().to(get_full_url::<I, R, Q>))
    );
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
    container: Data<Arc<Container<I, R, Q>>>,
    input: Json<CreateShortUrlRequest>,
) -> Result<impl Responder>
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortUrlRepository + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    let copy_url = input.url.as_str();
    let resp = container
    .command
    .execute(copy_url.to_owned())
    .await
    .map(|id| web::Json(ShortUrlResponse { url:id }))
    .expect("Dummy Error");
    
    Ok(resp)
}

#[derive(Serialize, Deserialize)]
struct FullUrlResponse {
    url: String
}

async fn get_full_url<I, R, Q>(
    container: Data<Arc<Container<I, R, Q>>>,
    id: web::Path<String>
) -> Result<impl Responder>
where
    I: IDProvider + Send + Sync + 'static,
    R: CreateShortUrlRepository + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    let resp = container
    .query
    .execute(&id)
    .await
    .map(|id| web::Json(FullUrlResponse { url:id }))
    .expect("Dummy Error");

    Ok(resp)

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        dependency_injection::{self, Container, DC},
        id_provider::FakeIDProvide,
        storage::CreateShortUrlRepository,
    };
    use actix_web::{http::StatusCode, test};
    use dashmap::DashMap;
    use std::sync::Arc;

    fn get_fouter_with_mock_container<I, R, Q>(
    ) -> Arc<Container<FakeIDProvide, StorageRepository, StorageRepository>>
    where
        I: IDProvider + Send + Sync + 'static,
        R: CreateShortUrlRepository + Send + Sync + 'static,
        Q: GetFullUrlRepository + Send + Sync + 'static,
    {
        let store = Arc::new(DashMap::new());
        store.insert("test-id_1".to_owned(), "test_url_1".to_owned());
        store.insert("test-id_2".to_owned(), "test_url_2".to_owned());
        let repo = StorageRepository::new(store);

        let container = <dependency_injection::Container<
            FakeIDProvide,
            StorageRepository,
            StorageRepository,
        >>::new(
            FakeIDProvide::new("test-id_1".to_owned()),
            repo.clone(),
            repo,
        );

        Arc::new(container)
    }

    #[actix_rt::test]
    async fn tes_get_full_url_success() {
        //given
        let container =
            get_fouter_with_mock_container::<FakeIDProvide, StorageRepository, StorageRepository>();

        let mut app = test::init_service(App::new().app_data(web::Data::new(container)).route(
            "/{id}",
            web::get().to(get_full_url::<FakeIDProvide, StorageRepository, StorageRepository>),
        ))
        .await;
        let req = test::TestRequest::get().uri("/test-id_1").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn tes_shorten_url_success() {
        //given
        let container =
            get_fouter_with_mock_container::<FakeIDProvide, StorageRepository, StorageRepository>();
        let json = CreateShortUrlRequest {
            url: "test_url_1".to_owned(),
        };

        let mut app = test::init_service(App::new().app_data(web::Data::new(container)).route(
            "/",
            web::post().to(shorten_url::<FakeIDProvide, StorageRepository, StorageRepository>),
        ))
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(json)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_get_full_url() {
        //given
        let container =
            get_fouter_with_mock_container::<FakeIDProvide, StorageRepository, StorageRepository>();

        let mut app = test::init_service(App::new().app_data(web::Data::new(container)).route(
            "/{id}",
            web::get().to(get_full_url::<FakeIDProvide, StorageRepository, StorageRepository>),
        ))
        .await;
        let req = test::TestRequest::get().uri("/test-id_1").to_request();
        let resp: FullUrlResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!("test_url_1".to_owned(), resp.url);
    }

    #[actix_rt::test]
    async fn test_shorten_url() {
        //given
        let container =
            get_fouter_with_mock_container::<FakeIDProvide, StorageRepository, StorageRepository>();
        let json = CreateShortUrlRequest {
            url: "https://yandex.ru".to_owned(),
        };

        let mut app = test::init_service(App::new().app_data(web::Data::new(container)).route(
            "/",
            web::post().to(shorten_url::<FakeIDProvide, StorageRepository, StorageRepository>),
        ))
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(json)
            .to_request();
        let resp: ShortUrlResponse = test::call_and_read_body_json(&app, req).await;
        assert_eq!("test-id_1", resp.url);
    }
}