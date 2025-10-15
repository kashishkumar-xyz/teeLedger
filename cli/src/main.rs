use clap::{Parser, Subcommand};

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

    let db_path = cli.database.unwrap_or_else(|| "./ledger.db".to_string());
    println!("Using database: {}", db_path);

    match &cli.command {
        Commands::Add {
            person,
            amount,
            date,
            note,
        } => {
            println!(
                "Adding transaction: Person={}, Amount={}, Date={:?}, Note={:?}",
                person, amount, date, note
            );
        }
        Commands::List { person } => {
            println!("Listing transactions for person: {:?}", person);
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
}
