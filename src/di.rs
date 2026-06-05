use crate::app::query::get_hello_world::{GetHelloWorldQuery, Repository};

pub struct Container<R>
where
    R: Repository,
{
    pub hello_world_query: GetHelloWorldQuery<R>,
}

impl<R> Container<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self {
            hello_world_query: GetHelloWorldQuery::new(repository),
        }
    }
}
