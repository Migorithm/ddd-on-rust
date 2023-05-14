use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::DomainEvent;

#[async_trait]
pub trait Aggregate: Default + Serialize + DeserializeOwned + Sync + Send {
    type Command: Send;
    type Event: DomainEvent<Self>;
    type Error: std::error::Error;
    type Services: Send + Sync;

    /// Aggregate type is identifier for this aggregate
    fn aggregate_type() -> String;

    fn aggregate_version(&self) -> String;

    /// Execute command on this aggregate
    /// The result is either vector of events or error
    async fn execute(
        &mut self,
        command: Self::Command,
        _service: &Self::Services,
    ) -> Result<(), Self::Error> {
        let event = Self::convert_command(command);
        self.trigger(event);
        Ok(())
    }

    fn convert_command(command: Self::Command) -> Self::Event;

    /// Trigger domain event of given type
    fn trigger(&mut self, event: <Self as crate::aggregate::Aggregate>::Event) {
        event.mutate(Some(self));
    }

    fn create(command: Self::Command) -> Option<Self>;

    //TODO Versioning
    //TODO Collecting Event

}
