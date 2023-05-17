use crate::aggregate::Aggregate;
use crate::AggregateError;
use async_trait::async_trait;

/// The abstract central source for loading past events and committing new events.
#[async_trait]
pub trait Repository<A>: Send + Sync
where
    A: Aggregate,
{
    /// Add aggregate to a session
    /// If underlying db doesn't have the key, then addition gets successful, returning id.
    async fn add(&mut self, aggregate: &A) -> Result<String, AggregateError>;

    /// Load aggregate at current state
    async fn get(&self, aggregate_id: &str) -> Result<A, AggregateError>;
}
