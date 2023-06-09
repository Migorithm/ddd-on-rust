use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt;

use crate::Aggregate;

/// `DomainEvent` which is specific to the domain
pub trait DomainEvent<A>:
    Serialize + DeserializeOwned + Clone + PartialEq + fmt::Debug + Sync + Send
where
    A: Aggregate,
{
    /// To get event name, maybe used for upcasting
    fn event_type(&self) -> String;

    /// To get event version, maybe used for upcasting
    fn event_version(&self) -> String;

    /// To update aggregate's state
    fn apply(&self, aggregate: Option<&mut A>) -> Option<A>;

    fn mutate(&self, aggregate: Option<&mut A>) -> Option<A> {
        // TODO implement version control

        self.apply(aggregate)
    }
}

// `EventWrapper` is to wrap an event with its relavent information
// Within the bounded context, the following set must be unique
// - `aggregate_id`
// - `version`

// #[derive(Debug)]
// pub struct EventWrapper {
//     pub aggregate_id: String,
//     pub version: usize,
// }

// impl Clone for EventWrapper {
//     fn clone(&self) -> Self {
//         Self {
//             aggregate_id: self.aggregate_id.clone(),
//             version: self.version,
//         }
//     }
// }
