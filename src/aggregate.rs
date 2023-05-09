use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::DomainEvent;

#[async_trait]
pub trait Aggregate: Default + Serialize + DeserializeOwned + Sync + Send {
    type Command;
    type Event: DomainEvent;
    type Error: std::error::Error;
    type Services: Send + Sync;

    /// Aggregate type is identifier for this aggregate
    fn aggregate_type() -> String;

    /// Execute command on this aggregate
    /// The result is either vector of events or error
    async fn execute(
        &self,
        command: Self::Command,
        service: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error>;

    /// To update aggregate's state
    /// ```rust
    /// fn apply(&mut self, event:Self::Event){    
    ///     event.apply(self)    
    /// }
    /// ```
    fn apply(&mut self, event: Self::Event);
}
