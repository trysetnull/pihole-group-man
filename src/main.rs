use clap::{Args, Parser, Subcommand};
use log::{info, LevelFilter};
use rusqlite::{Connection, Result};
use simple_logger::SimpleLogger;
use std::process;

/// Simple program to administrate groups of a client.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Filepath to the pi-hole gravity database.
    #[arg(long, default_value = "/etc/pihole/gravity.db")]
    database_path: String,

    /// Make the operation more talkative.
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Silent mode.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    silent: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Appends client to group.
    Append(ClientGroupArgs),

    /// Removes client from group.
    Remove(ClientGroupArgs),
}

#[derive(Args, Debug)]
struct ClientGroupArgs {
    /// Comment used to identify the client.
    #[arg(long, default_value = "Fire TV cube")]
    client_comment: String,

    /// Name used to identify the group.
    #[arg(long, default_value = "Unresolved")]
    group_name: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut filter = match cli.verbose {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    if cli.silent {
        filter = LevelFilter::Off;
    }

    SimpleLogger::new().with_level(filter).init().unwrap();

    // Open a connection to the database
    let conn = Connection::open(cli.database_path)?;

    match &cli.command {
        Commands::Append(args) => {
            if let Err(_error) =
                pihole_group_man::append(&conn, &args.client_comment, &args.group_name)
            {
                process::exit(1);
            }
            info!("SUCCESS");
            Ok(())
        }
        Commands::Remove(args) => {
            if let Err(_error) =
                pihole_group_man::remove(&conn, &args.client_comment, &args.group_name)
            {
                process::exit(1);
            }
            info!("SUCCESS");
            Ok(())
        }
    }
}
