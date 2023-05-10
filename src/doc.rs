use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{Aggregate, AggregateError, DomainEvent};

//VO
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Item {
    //TODO type state pattern
    product_id: String,
    name: String,
    sell_price: usize,
    status: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum TransactionEvent {
    PurchaseMade(Vec<Item>),
    CancellationRequested(Item),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionCommand {
    MakePurchase(Vec<Item>),
    RequestCancellation(Item),
}

impl DomainEvent for TransactionEvent {
    type Aggregate = Transaction;

    fn event_type(&self) -> String {
        match self {
            Self::PurchaseMade(_) => "PurchaseMade".to_string(),
            Self::CancellationRequested(_) => "CancellationRequested".to_string(),
        }
    }
    fn event_version(&self) -> String {
        //For Upcasting
        "1.0.0".to_string()
    }

    fn apply(&self, aggregate: &mut Self::Aggregate) {
        match self {
            Self::CancellationRequested(item) => {
                if let Some(idx) = aggregate.items.iter().position(|ele| ele == item) {
                    let item = aggregate.items.get_mut(idx).unwrap();
                    item.status = "cancelled".to_string();
                }
            }
            _ => (), // passing
        }
    }
}

//Tranasction Aggregate
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Transaction {
    items: Vec<Item>,
    pay_amount: usize,
}

#[async_trait]
impl Aggregate for Transaction {
    type Command = TransactionCommand;
    type Event = TransactionEvent;
    type Error = AggregateError;
    type Services = TransactionService;

    fn aggregate_type() -> String {
        todo!()
    }
    fn apply(&mut self, event: Self::Event) {
        todo!()
    }

    async fn execute(
        &self,
        command: Self::Command,
        service: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        todo!()
    }
}

pub struct TransactionService;
