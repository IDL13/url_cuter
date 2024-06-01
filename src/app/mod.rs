pub mod command;
pub mod query;

#[cfg(test)]
mod tests {
    use dashmap::DashMap;
    use std::sync::Arc;

    use super::command::create_short_url::CreateShortUrlCommand;
    use super::query::get_full_url::GetFullUrlQuery;
    use crate::storage::StorageRepository;

    #[tokio::test]
    async fn create_and_get_short_url() {
        //given
        let store = Arc::new(DashMap::new());
        let repo = StorageRepository::new(store);

        let command = CreateShortUrlCommand::new(
            crate::id_provider::NanoIDProvider,
            repo.clone());

        let get_query = GetFullUrlQuery::new(repo);

        //when
        let result = command.execute("https://www.google.com".to_owned()).await;

        let result_1 = get_query.execute(&result.unwrap()).await.unwrap();

        //then
        assert_eq!(result_1, "https://www.google.com".to_owned());
    }
}