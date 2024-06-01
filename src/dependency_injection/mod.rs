use command::create_short_url::CreateShortUrlCommand;
use query::get_full_url::{GetFullUrlQuery, GetFullUrlRepository};

use crate::{app::*,
    id_provider::{IDProvider}, storage::CreateShortUrlRepository
};

pub trait DC<I, R, Q>
where
    I: IDProvider,
    R: CreateShortUrlRepository,
    Q: GetFullUrlRepository
{
    fn new(id_provider: I, repository: R, querier: Q) -> Container<I, R, Q>;
}
pub struct Container<I, R, Q>
where
    I: IDProvider,
    R: CreateShortUrlRepository,
    Q: GetFullUrlRepository,
{
    pub command: CreateShortUrlCommand<I, R>,
    pub query: GetFullUrlQuery<Q>,
}

impl<I, Q, R> DC<I, R, Q> for Container<I, R, Q>
where
    I: IDProvider,
    R: CreateShortUrlRepository,
    Q: GetFullUrlRepository
{
    fn new(id_provider: I, repository: R, querier: Q) -> Container<I, R, Q> {
        let command = CreateShortUrlCommand::new(id_provider, repository);
        let query = GetFullUrlQuery::new(querier);

        Self {
            command,
            query,
        }
    }
}
