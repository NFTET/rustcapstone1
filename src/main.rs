mod location;
use std::fs::File;
use std::io::{self,BufReader,BufRead};
mod transaction;
use transaction::Transaction;

fn main() {
    let file=File::open("./transactions.csv").unwrap();
    let reader=BufReader::new(file);
    let mut transactions:Vec<Transaction>=Vec::new();
    let mut skipped_lines: Vec<_>=Vec::new();
    for (idx, line) in reader.lines().enumerate(){
        if idx == 0{
            continue;
        }
        let line_str = line.unwrap();
        let parsed_transaction=Transaction::from_csv_line(&line_str);
        match parsed_transaction{
            Ok(transaction)=> transactions.push(transaction),
            Err(error_string)=>skipped_lines.push((idx,error_string,line_str))
        }
    }
    for transaction in transactions.iter() {
        println!("{:?}",transaction);
    }
    for skipped_line in skipped_lines.iter() {
        println!("{:?}",skipped_line);
    }
}
