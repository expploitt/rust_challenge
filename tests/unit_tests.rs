use rust_project_08::client;
use rust_project_08::transaction::{Transaction, TransactionType};

/// Argument tests
#[test]
fn is_csv() {
    let args = vec!["--".to_string(), "file.csv".to_string()];
    assert_eq!(rust_project_08::utils::parse_args(&args), true);
}

#[test]
fn no_csv() {
    let args = vec!["--".to_string(), "file.tar.gz".to_string()];
    assert_ne!(rust_project_08::utils::parse_args(&args), true);
}




/// Transaction operations
///
#[test]
fn deposit() {
    let mut client = client::Client::new(0);
    let tx0 = Transaction::new(TransactionType::Deposit, 0, 0, Some(2.0));
    client.deposit(&tx0);

    assert_eq!(client.available(), 2.0);
    assert_eq!(client.total(), 2.0);
    assert_eq!(client.held(), 0.0);
    assert_eq!(client.locked(), false);
}

#[test]
fn withdrawal() {
    let mut client = client::Client::new(0);
    let tx0 = Transaction::new(TransactionType::Deposit, 0, 0, Some(2.0));
    let tx1 = Transaction::new(TransactionType::Withdrawal, 0, 1, Some(1.0));

    client.deposit(&tx0);
    client.withdrawal(&tx1);

    assert_eq!(client.available(), 1.0);
    assert_eq!(client.total(), 1.0);
    assert_eq!(client.held(), 0.0);
    assert_eq!(client.locked(), false);
}

#[test]
fn dispute() {
    let mut client = client::Client::new(0);

    let tx0 = Transaction::new(TransactionType::Deposit, 0, 0, Some(2.0));
    let tx1 = Transaction::new(TransactionType::Deposit, 0, 1, Some(1.0));

    client.deposit(&tx0);
    client.deposit(&tx1);

    let _tx2 = Transaction::new(TransactionType::Dispute, 0, 1, None);
    client.dispute(Some(&tx1));

    assert_eq!(client.available(), 3.0 - 1.0);
    assert_eq!(client.total(), 3.0);
    assert_eq!(client.held(), 0.0 + 1.0);
    assert_eq!(client.locked(), false);

    assert_eq!(client.available(), client.total() - client.held());
    assert_eq!(client.total(), client.available() + client.held());
    assert_eq!(client.held(), client.total() - client.available());
}

#[test]
fn resolve() {
    let mut client = client::Client::new(0);

    let tx0 = Transaction::new(TransactionType::Deposit, 0, 0, Some(2.0));
    let tx1 = Transaction::new(TransactionType::Deposit, 0, 1, Some(1.0));

    client.deposit(&tx0);
    client.deposit(&tx1);

    let _tx2 = Transaction::new(TransactionType::Dispute, 0, 1, None);
    client.dispute(Some(&tx1));

    client.resolve(Some(&tx1));

    assert_eq!(client.available(), 2.0 + 1.0);
    assert_eq!(client.total(), 3.0);
    assert_eq!(client.held(), 1.0 - 1.0);
    assert_eq!(client.locked(), false);

    assert_eq!(client.available(), client.total() - client.held());
    assert_eq!(client.total(), client.available() + client.held());
    assert_eq!(client.held(), client.total() - client.available());
}

#[test]
fn chargeback() {
    let mut client = client::Client::new(0);

    let tx0 = Transaction::new(TransactionType::Deposit, 0, 0, Some(2.0));
    let tx1 = Transaction::new(TransactionType::Deposit, 0, 1, Some(1.0));

    client.deposit(&tx0);
    client.deposit(&tx1);

    let _tx2 = Transaction::new(TransactionType::Dispute, 0, 1, None);
    client.dispute(Some(&tx1));
    client.resolve(Some(&tx1));
    client.chargeback(Some(&tx1));

    assert_eq!(client.available(), 2.0 + 1.0);
    assert_eq!(client.total(), 3.0 - 1.0);
    assert_eq!(client.held(), 0.0 - 1.0);
    assert_eq!(client.locked(), true);

    assert_eq!(client.available(), client.total() - client.held());
    assert_eq!(client.total(), client.available() + client.held());
    assert_eq!(client.held(), client.total() - client.available());
}