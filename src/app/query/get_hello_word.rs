pub trait Repository {
    fn get_hello_world(&self) -> impl std::future::Future<Output = &'static str> + Send;
}

pub struct GetHelloWorldQuery<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> GetHelloWorldQuery<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> &'static str {
        self.repository.get_hello_world().await
    }
}

#[derive(Default)]
pub struct InMemoryRepository;

impl Repository for InMemoryRepository {
    async fn get_hello_world(&self) -> &'static str {
        "Hello, world!"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_hello_world_query() {
        let repository = InMemoryRepository;
        let query = GetHelloWorldQuery::new(repository);

        let result = query.execute().await;

        assert_eq!(result, "Hello, world!");
    }
}
