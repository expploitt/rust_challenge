use std::hash::{Hash, Hasher};
use serde::Deserialize;
use strum::EnumString;

#[derive(Clone, Debug, Deserialize, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub tx_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f32>,
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.tx == other.tx
    }
}

impl Eq for Transaction {}

impl Hash for Transaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tx.hash(state)
    }
}
impl Transaction {
    pub fn new(tx_type: TransactionType, client: u16, tx: u32, amount: Option<f32>) -> Self {
        Self {
            tx_type,
            client,
            tx,
            amount,
        }
    }

    pub fn format_4f(&mut self) {
        if self.amount.is_some() {
            self.amount = Some(((self.amount.unwrap() * 10000.0) as f32).floor() / 10000.0);
        }
    }
}