use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::fs::File;
use std::ops::Not;
use std::path::Path;
use csv::{Reader, Terminator};
use crate::client::Client;
use crate::transaction::{Transaction, TransactionType};


pub fn run(filename: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = read_file(filename)?;
    let c = process_transactions(&mut rdr)?;
    write_stdout(c)?;

    Ok(())
}

pub fn parse_args<'a>(args: &Vec<String>) -> bool {
    if args.len() < 1 {
        return false;
    }
    let input_file = &args[1];

    Path::new(input_file).extension().and_then(OsStr::to_str).eq(&Some("csv"))
}

pub fn read_file(file: String) -> Result<Reader<File>, Box<dyn std::error::Error>> {
    let input_file = File::open(file)?;

    Ok(csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(input_file))
}

pub fn process_transactions(rdr: &mut Reader<File>) -> Result<HashMap<u16, Client>, Box<dyn std::error::Error>> {
    let mut clients = HashMap::<u16, Client>::new();
    let mut transactions = HashSet::<Transaction>::new();

    let headers = rdr.headers().unwrap().clone();
    let mut records = rdr.records();

    while let Some(r) = records.next() {
        if let Ok(r) = r {
            let mut tx = r.deserialize::<Transaction>(Some(&headers))?;
            tx.format_4f();
            clients
                .contains_key(&tx.client)
                .not()
                .then(||
                    clients.insert(tx.client, Client::new(tx.client))
                );

            let client = clients.get_mut(&tx.client).unwrap();

            match &tx.tx_type {
                TransactionType::Deposit => {
                    client.deposit(&tx);
                    transactions.insert(tx);
                }
                TransactionType::Withdrawal => {
                    client.withdrawal(&tx);
                    transactions.insert(tx);
                }
                TransactionType::Dispute => {
                    client.dispute(transactions.get(&tx));
                }
                TransactionType::Resolve => {
                    client.resolve(transactions.get(&tx));
                }
                TransactionType::Chargeback => {
                    client.chargeback(transactions.get(&tx));
                }
            }
        }
    }

    Ok(clients)
}

pub fn write_stdout(clients: HashMap<u16, Client>) -> Result<(), Box<dyn std::error::Error>> {
    let mut wrt = csv::WriterBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .terminator(Terminator::CRLF)
        .from_writer(std::io::stdout());

    for (_id, mut client) in clients {
        client.format_4f();
        wrt.serialize(client)?;
        wrt.flush()?;
    }

    Ok(())
}


