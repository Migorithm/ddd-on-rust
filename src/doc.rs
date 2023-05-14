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
    CancellationRequested{transaction_id:String,product_id:String},
    FullCancellationRequested
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionCommand {
    MakePurchase(Vec<Item>),
    RequestCancellation{
        transaction_id:String,
        product_id:String
    },
}

impl DomainEvent<Transaction> for TransactionEvent {
    fn event_type(&self) -> String {
        match self {
            Self::PurchaseMade(_) => "PurchaseMade".to_string(),
            Self::CancellationRequested{transaction_id,product_id} => "CancellationRequested".to_string(),
            Self::FullCancellationRequested => "FullCancellationRequested".to_string(),
        }
    }
    fn event_version(&self) -> String {
        //For Upcasting
        "1.0.0".to_string()
    }

    fn apply(&mut self, aggregate: &mut Transaction) {
        match self {
            Self::CancellationRequested{transaction_id,product_id} => {
                if let Some(idx) = aggregate.items.iter().position(|ele| ele.product_id == product_id.as_ref()) {
                    let item = aggregate.items.get_mut(idx).unwrap();
                    item.status = "cancelled".to_string();
                }
            }
            Self::FullCancellationRequested => {
                aggregate.items.iter_mut().for_each(|i| i.status = "cancelled".to_string())
            }
            _ => (),
        }
    }
}

//Tranasction Aggregate
#[derive(Debug, Default, Serialize, Deserialize)]
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

    fn aggregate_version(&self) -> String{
        self.id.clone()
    }

    fn convert_command(command: Self::Command) -> Result<Self::Event, Self::Error> {
        match command {
            Self::Command::MakePurchase(items) => Ok(Self::Event::PurchaseMade(items)),
            Self::Command::RequestCancellation{transaction_id,product_id} => {
                Ok(Self::Event::CancellationRequested{transaction_id,product_id})
            }
        }
    }
    
    fn create(command:Self::Command)->Self{
        if let Self::Command::MakePurchase(items) = command{
            return Self{
                id: Uuid::new_v4().to_string(),
                pay_amount:items.iter().map(|i|i.sell_price).sum(),
                items
            }
        }else{
            panic!("Only Make Purchase Command is allowed!")
        }
        
    }
}

pub struct TransactionService;


#[cfg(test)]
mod doc_tests {
    use super::*;

    fn create_cmd_helper()->TransactionCommand {
        TransactionCommand::MakePurchase(vec![
            Item{
                product_id: "1".to_string(),
                name: "shoes".to_string(),
                sell_price: 30000,
                status:"created".to_string()
            },
            Item{
                product_id: "2".to_string(),
                name: "phone".to_string(),
                sell_price: 550000,
                status:"created".to_string()
            },
            ])
    }

    #[test]
    fn test_create_transaction(){
        let cmd = create_cmd_helper();
        let transaction = Transaction::create(cmd);
        println!("{:?}",transaction);
    }

    #[tokio::test]
    async fn test_cancel_on_item(){
        let cmd = create_cmd_helper();
        let mut transaction = Transaction::create(cmd);
        let cancel_cmd = TransactionCommand::RequestCancellation{
            transaction_id:transaction.aggregate_version(),
            product_id:"1".to_string()
        };
        let res = transaction.execute(cancel_cmd, &TransactionService).await;
        matches!(res,Ok(()));


        let item_to_be_canceled = &transaction.items[0] ;
        assert_eq!(item_to_be_canceled.status,"cancelled".to_string());
    }
}