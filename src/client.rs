use std::collections::HashSet;
use serde::{Serialize};
use crate::transaction::Transaction;

#[derive(Clone, Debug, Serialize)]
pub struct Client {
    client: u16,
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
    #[serde(skip_serializing)]
    disputed_tx: HashSet<u32>,
    #[serde(skip_serializing)]
    resolved_tx: HashSet<u32>,
}

impl Client {
    pub fn new(id: u16) -> Self {
        Self {
            client: id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
            disputed_tx: HashSet::<u32>::new(),
            resolved_tx: HashSet::<u32>::new(),
        }
    }

    /// A deposit is a credit to the client's asset account, meaning it should increase the available and
    /// total funds of the client account
    ///
    pub fn deposit(&mut self, tx: &Transaction) {
        let deposit = tx.amount.unwrap();
        self.available += deposit;
        self.total += deposit;
    }

    /// A withdraw is a debit to the client's asset account, meaning it should decrease the available and
    /// total funds of the client account
    ///
    /// If a client does not have sufficient available funds the withdrawal should fail and the total amount
    /// of funds should not change
    ///
    pub fn withdrawal(&mut self, tx: &Transaction) {
        let withdrawal = tx.amount.unwrap();

        if self.available >= withdrawal {
            self.available -= withdrawal;
            self.total -= withdrawal;
        }
    }

    /// A dispute represents a client's claim that a transaction was erroneous and should be reversed.
    /// The transaction shouldn't be reversed yet but the associated funds should be held. This means
    /// that the clients available funds should decrease by the amount disputed, their held funds should
    /// increase by the amount disputed, while their total funds should remain the same.
    ///
    pub fn dispute(&mut self, disputed_tx: Option<&Transaction>) {
        if let Some(tx) = disputed_tx {
            let dispute = tx.amount.unwrap();
            self.available -= dispute;
            self.held += dispute;
            self.disputed_tx.insert(tx.tx);
        }
    }

    /// A resolve represents a resolution to a dispute, releasing the associated held funds. Funds that
    /// were previously disputed are no longer disputed. This means that the clients held funds should
    /// decrease by the amount no longer disputed, their available funds should increase by the
    /// amount no longer disputed, and their total funds should remain the same.
    ///
    pub fn resolve(&mut self, resolved_tx: Option<&Transaction>) {
        if let Some(tx) = resolved_tx {
            if self.disputed_tx.contains(&tx.tx) {
                let resolve = tx.amount.unwrap();
                self.held -= resolve;
                self.available += resolve;
                self.disputed_tx.remove(&tx.tx);
                self.resolved_tx.insert(tx.tx);
            }
        }
    }

    /// A chargeback is the final state of a dispute and represents the client reversing a transaction.
    /// Funds that were held have now been withdrawn. This means that the clients held funds and
    /// total funds should decrease by the amount previously disputed. If a chargeback occurs the
    /// client's account should be immediately frozen.
    ///
    pub fn chargeback(&mut self, chargeback_tx: Option<&Transaction>) {
        if let Some(tx) = chargeback_tx {
            // if the tx was previously resolved
            if self.resolved_tx.contains(&tx.tx) {
                let chargeback = tx.amount.unwrap();
                self.held -= chargeback;
                self.total -= chargeback;
                self.locked = true;
                self.resolved_tx.remove(&tx.tx);
            }
        }
    }

    pub fn format_4f(&mut self) {
        self.available = ((self.available * 10000.0) as f32).floor() / 10000.0;
        self.held = ((self.held * 10000.0) as f32).floor() / 10000.0;
        self.total = ((self.total * 10000.0) as f32).floor() / 10000.0;

    }
    pub fn id(&self) -> u16 {
        self.client
    }
    pub fn available(&self) -> f32 {
        self.available
    }
    pub fn held(&self) -> f32 {
        self.held
    }
    pub fn total(&self) -> f32 {
        self.total
    }
    pub fn locked(&self) -> bool {
        self.locked
    }
}