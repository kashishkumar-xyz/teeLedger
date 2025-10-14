// cli/src/main.rs
// This file will contain the main logic for the CLI application.

use clap::{Parser, Subcommand};

/// A secure, offline-first personal ledger application.
#[derive(Parser, Debug)]
#[command(author, version, about = "A secure, offline-first personal ledger application for tracking financial transactions and balances.", long_about = None)]
struct Cli {
    /// Optional path to the database file. Defaults to a standard location.
    #[arg(short, long, value_name = "FILE")]
    database: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Adds a new transaction to the ledger
    Add { 
        /// The person involved in the transaction
        #[arg(short, long)]
        person: String,
        /// The amount of the transaction (e.g., 100 for $1.00)
        #[arg(short, long)]
        amount: i64,
        /// The date of the transaction (YYYY-MM-DD). Defaults to today.
        #[arg(short, long)]
        date: Option<String>,
        /// A note or description for the transaction
        #[arg(short, long)]
        note: Option<String>,
    },
    /// Lists all transactions or transactions for a specific person
    List { 
        /// Optional: List transactions for a specific person
        #[arg(short, long)]
        person: Option<String>,
    },
    /// Shows the balance for all people or a specific person
    Balance { 
        /// Optional: Show balance for a specific person
        #[arg(short, long)]
        person: Option<String>,
    },
    /// Creates an encrypted backup of the database
    Backup { 
        /// Path to save the backup file
        #[arg(short, long)]
        output: String,
    },
    /// Restores the database from an encrypted backup
    Restore { 
        /// Path to the backup file to restore from
        #[arg(short, long)]
        input: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // Placeholder for database initialization/opening logic
    let db_path = cli.database.unwrap_or_else(|| "./ledger.db".to_string());
    println!("Using database: {}", db_path);

    match &cli.command {
        Commands::Add { person, amount, date, note } => {
            println!("Adding transaction: Person={}, Amount={}, Date={:?}, Note={:?}", person, amount, date, note);
            // Call core library function here
        },
        Commands::List { person } => {
            println!("Listing transactions for person: {:?}", person);
            // Call core library function here
        },
        Commands::Balance { person } => {
            println!("Showing balance for person: {:?}", person);
            // Call core library function here
        },
        Commands::Backup { output } => {
            println!("Creating backup to: {}", output);
            // Call core library function here
        },
        Commands::Restore { input } => {
            println!("Restoring from backup: {}", input);
            // Call core library function here
        },
    }
}
