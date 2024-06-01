use crate::id_provider::{self, IDProvider};
use crate::storage::CreateShortUrlRepository;

pub struct  CreateShortUrlCommand<I, R>
where
    I: IDProvider,
    R: CreateShortUrlRepository,
{
    id_provider: I,
    repo: R,
}

impl<I, R> CreateShortUrlCommand<I, R>
where
    I: IDProvider,
    R: CreateShortUrlRepository,
{
    pub fn new(id_provider: I, repo: R) -> Self {
        Self { id_provider, repo}
    }

    pub async fn execute(&self, full_url: String) -> Result<String, String> {
        let id = self.id_provider.provide();

        self.repo.save(full_url, id.clone())?;

        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use dashmap::DashMap;

    use crate::{app::command, storage::{self, StorageRepository}};

    use super::*;

    #[tokio::test]
    async fn get_short_url() {
        //given
        let id_provider = crate::id_provider::FakeIDProvide::new("123".to_owned());
        let store = Arc::new(DashMap::new());
        let repo = StorageRepository::new(store);
        let command = CreateShortUrlCommand::new(id_provider, repo);

        //res
        let result = command.execute("https://www.yandex.com".to_owned()).await;

        //then
        assert_ne!(result, Ok("".to_owned()))
    }

    #[tokio::test]
    async fn get_two_different_url() {
        //given
        let id_provider = crate::id_provider::NanoIDProvider;
        let store = Arc::new(DashMap::new());
        let repo = StorageRepository::new(store);
        let command = CreateShortUrlCommand::new(id_provider, repo);

        //res
        let result_1 = command.execute("https://www.yandex.com".to_owned()).await;

        let result_2 = command.execute("https://www.yandex.com".to_owned()).await;

        //then
        assert_ne!(result_1, result_2)
    }

    #[tokio::test]
    async fn save_one_item() {
        //given
        let id_provider = crate::id_provider::NanoIDProvider;
        let store = Arc::new(DashMap::new());
        let repo = StorageRepository::new(store.clone());
        let command = CreateShortUrlCommand::new(id_provider, repo);

        //when
        let id = command
            .execute("https://www.yandex.com".to_owned())
            .await
            .unwrap();

        //then
        assert_eq!(store.len(), 1);

        let full_url = store.get(&id).unwrap();
        assert_eq!(full_url.value(), "https://www.yandex.com")
    }
}

