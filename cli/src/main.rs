use clap::{Parser, Subcommand};
use teeLedger::ffi::{add_transaction_ffi, list_transactions_ffi, free_transactions_ffi};
use teeLedger::key_management::generate_dek;
use zeroize::Zeroize;
use std::ffi::{CStr, CString};
use std::ptr;
use chrono::Local;
use teeLedger::models::Transaction;

#[derive(Parser, Debug)]
#[command(author, version, about = "A secure, offline-first personal ledger application for tracking financial transactions and balances.", long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    database: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        #[arg(short, long)]
        person: String,

        #[arg(short, long)]
        amount: i64,

        #[arg(short, long)]
        date: Option<String>,

        #[arg(short, long)]
        note: Option<String>,
    },

    List {
        #[arg(short, long)]
        person: Option<String>,
    },

    Balance {
        #[arg(short, long)]
        person: Option<String>,
    },

    Backup {
        #[arg(short, long)]
        output: String,
    },

    Restore {
        #[arg(short, long)]
        input: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let db_path_str = cli.database.unwrap_or_else(|| "./ledger.db".to_string());
    let db_path = CString::new(db_path_str.as_str()).unwrap();

    let mut dek = generate_dek();
    let key_len = dek.len() as i32;

    match &cli.command {
        Commands::Add {
            person,
            amount,
            date,
            note,
        } => {
            let final_date = date
                .as_ref()
                .map_or_else(|| Local::now().format("%Y-%m-%d").to_string(), |d| d.clone());

            let c_date = CString::new(final_date.as_str()).unwrap();
            let c_person = CString::new(person.as_str()).unwrap();
            let c_note = note.as_ref().map_or(ptr::null(), |s| unsafe { CString::new(s.as_str()).unwrap().into_raw() });

            let result = unsafe {
                add_transaction_ffi(
                    db_path.as_ptr(),
                    dek.as_mut_ptr(),
                    key_len,
                    c_date.as_ptr(),
                    c_person.as_ptr(),
                    *amount,
                    c_note,
                )
            };

            if result == 0 {
                println!("Transaction added successfully.");
            } else {
                eprintln!("Error adding transaction.");
            }
            unsafe { ptr::drop_in_place(c_note); }
        }
        Commands::List { person } => {
            let c_person = person.as_ref().map_or(ptr::null(), |s| unsafe { CString::new(s.as_str()).unwrap().into_raw() });
            let mut count: c_int = 0;
            let transactions_ptr = unsafe {
                list_transactions_ffi(
                    db_path.as_ptr(),
                    dek.as_mut_ptr(),
                    key_len,
                    c_person,
                    &mut count,
                )
            };

            if transactions_ptr.is_null() {
                eprintln!("Error listing transactions.");
            } else {
                let transactions_slice = unsafe { std::slice::from_raw_parts(transactions_ptr, count as usize) };
                if count == 0 {
                    println!("No transactions found.");
                } else {
                    println!("Transactions:");
                    for i in 0..count as usize {
                        let c_str = unsafe { CStr::from_ptr(transactions_slice[i]) };
                        let json_str = c_str.to_str().unwrap();
                        let transaction: Transaction = serde_json::from_str(json_str).unwrap();
                        println!("  {:?}", transaction);
                    }
                }
                unsafe { free_transactions_ffi(transactions_ptr, count) };
            }
            unsafe { ptr::drop_in_place(c_person); }
        }
        Commands::Balance { person } => {
            println!("Showing balance for person: {:?}", person);
        }
        Commands::Backup { output } => {
            println!("Creating backup to: {}", output);
        }
        Commands::Restore { input } => {
            println!("Restoring from backup: {}", input);
        }
    }
    dek.zeroize();
}
