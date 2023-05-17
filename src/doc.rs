use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{Aggregate, AggregateError, DomainEvent};
use uuid::Uuid;
//VO
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Item {
    product_id: String,
    name: String,
    sell_price: usize,
    status: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum TransactionEvent {
    PurchaseMade(Vec<Item>),
    CancellationRequested {
        transaction_id: String,
        product_id: String,
    },
    FullCancellationRequested,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionCommand {
    MakePurchase(Vec<Item>),
    RequestCancellation {
        transaction_id: String,
        product_id: String,
    },
}

impl DomainEvent<Transaction> for TransactionEvent {
    fn event_type(&self) -> String {
        match self {
            Self::PurchaseMade(_) => "PurchaseMade".to_string(),
            Self::CancellationRequested {
                transaction_id: _,
                product_id: _,
            } => "CancellationRequested".to_string(),
            Self::FullCancellationRequested => "FullCancellationRequested".to_string(),
        }
    }
    fn event_version(&self) -> String {
        //For Upcasting
        "1.0.0".to_string()
    }

    fn apply(&self, aggregate: Option<&mut Transaction>) -> Option<Transaction> {
        match self {
            Self::PurchaseMade(items) => {
                return Some(Transaction {
                    id: Uuid::new_v4().to_string(),
                    pay_amount: items.iter().map(|i| i.sell_price).sum(),
                    items: items.to_vec(),
                })
            }
            Self::CancellationRequested {
                transaction_id: _,
                product_id,
            } => {
                let agg = aggregate.unwrap();
                if let Some(idx) = agg
                    .items
                    .iter()
                    .position(|ele| ele.product_id == product_id.as_ref())
                {
                    let item = agg.items.get_mut(idx).unwrap();
                    item.status = "cancelled".to_string();
                }
                None
            }
            Self::FullCancellationRequested => {
                let agg = aggregate.unwrap();
                agg.items
                    .iter_mut()
                    .for_each(|i| i.status = "cancelled".to_string());
                None
            }
        }
    }
}

//Tranasction Aggregate
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Transaction {
    id: String, // Aggregate ID
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
        "Transaction".to_string()
    }

    fn aggregate_version(&self) -> String {
        self.id.clone()
    }

    fn convert_command(command: Self::Command) -> Self::Event {
        match command {
            Self::Command::MakePurchase(items) => Self::Event::PurchaseMade(items),
            Self::Command::RequestCancellation {
                transaction_id,
                product_id,
            } => Self::Event::CancellationRequested {
                transaction_id,
                product_id,
            },
        }
    }

    fn create(command: Self::Command) -> Option<Self> {
        if let Self::Command::MakePurchase(_) = command {
            let event = Self::convert_command(command);
            event.mutate(None)
        } else {
            None
        }
    }
}

pub struct TransactionService;

#[cfg(test)]
mod doc_tests {

    use std::collections::HashMap;

    use crate::Repository;

    use super::*;

    fn create_cmd_helper() -> TransactionCommand {
        TransactionCommand::MakePurchase(vec![
            Item {
                product_id: "1".to_string(),
                name: "shoes".to_string(),
                sell_price: 30000,
                status: "created".to_string(),
            },
            Item {
                product_id: "2".to_string(),
                name: "phone".to_string(),
                sell_price: 550000,
                status: "created".to_string(),
            },
        ])
    }

    #[test]
    fn test_create_transaction() {
        let cmd = create_cmd_helper();
        let transaction = Transaction::create(cmd);
        println!("{:?}", transaction);
    }

    #[tokio::test]
    async fn test_cancel_on_item() {
        let cmd = create_cmd_helper();
        let mut transaction = Transaction::create(cmd).expect("Must Be Passed");

        let cancel_cmd = TransactionCommand::RequestCancellation {
            transaction_id: transaction.aggregate_version(),
            product_id: "1".to_string(),
        };
        let res = transaction.execute(cancel_cmd, &TransactionService).await;
        matches!(res, Ok(()));

        let item_to_be_canceled = &transaction.items[0];
        assert_eq!(item_to_be_canceled.status, "cancelled".to_string());
    }

    #[tokio::test]
    async fn test_repository() {
        /// Examplify Repository Pattern

        #[derive(Default)]
        struct TransactionRepository {
            conn: HashMap<String, Transaction>,
        }

        #[async_trait]
        impl Repository<Transaction> for TransactionRepository {
            async fn add(&mut self, aggregate: &Transaction) -> Result<String, AggregateError> {
                self.conn
                    .entry(aggregate.id.clone())
                    .or_insert(aggregate.clone());
                Ok(aggregate.id.clone())
            }

            /// Load aggregate at current state
            async fn get(&self, aggregate_id: &str) -> Result<Transaction, AggregateError> {
                self.conn
                    .get(aggregate_id)
                    .map_or(Err(AggregateError::NotFound), |r| Ok(r.clone()))
            }
        }

        let mut trx_repo = TransactionRepository::default();

        let cmd = create_cmd_helper();
        let mut transaction = Transaction::create(cmd).expect("Must Be Passed");

        if let Err(err) = trx_repo.add(&transaction).await {
            assert_eq!(true, false);
        }

        if let Ok(trx) = trx_repo.get(&transaction.id).await {
            assert_eq!(trx, transaction)
        } else {
            assert_eq!(true, false);
        }
    }
}
