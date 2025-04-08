use clap::{Args, Parser, Subcommand};
use log::{info, LevelFilter};
use pihole::api::PiHoleV6Client;
use simple_logger::SimpleLogger;
use std::fs;

/// Simple program to administrate groups of a client.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// PiHole
    #[arg(long, default_value = "http://pi.hole:8080")]
    base_url: String,

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // Read password from systemd credential
    let password_path = std::env::var("CREDENTIALS_DIRECTORY")
        .expect("Systemd credentials not available")
        + "/pihole-password";

    let password = fs::read_to_string(&password_path)
        .unwrap_or_else(|_| panic!("Failed to read password from {}", password_path));

    let mut client = PiHoleV6Client::new(&cli.base_url);
    client.login(&password).await?;

    // Execute command
    match &cli.command {
        Commands::Append(args) => handle_group_action(&mut client, args, true).await?,
        Commands::Remove(args) => handle_group_action(&mut client, args, false).await?,
    }

    client.logout().await?;
    Ok(())
}

async fn handle_group_action(
    client: &mut PiHoleV6Client,
    args: &ClientGroupArgs,
    add: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get target group ID
    let groups = client.get_group(&args.group_name).await?;
    let group_entry = groups
        .groups
        .first()
        .ok_or(format!("Group '{}' not found", args.group_name))?;

    // Find client by comment
    let clients = client.get_clients().await?;
    let client_entry = clients
        .clients
        .iter()
        .find(|c| c.comment == args.client_comment)
        .ok_or(format!("Client '{}' not found", args.client_comment))?;

    // Update client groups
    let mut new_groups = client_entry.groups.clone();
    if add {
        if !new_groups.contains(&group_entry.id) {
            new_groups.push(group_entry.id);
            info!("Adding client to group '{}'", args.group_name);
        } else {
            info!("Client already in group '{}'", args.group_name);
            return Ok(());
        }
    } else if let Some(pos) = new_groups.iter().position(|&g| g == group_entry.id) {
        new_groups.remove(pos);
        info!("Removing client from group '{}'", args.group_name);
    } else {
        info!("Client not in group '{}'", args.group_name);
        return Ok(());
    }

    // Apply changes
    client
        .update_client(
            &client_entry.id.to_string(),
            client_entry.comment.clone(),
            new_groups,
        )
        .await?;

    info!("Successfully updated client '{}'", args.client_comment);
    Ok(())
}

// [Unit]
// Description=Pi-hole CLI Service
// After=network.target

// [Service]
// LoadCredentialEncrypted=pihole-password:/etc/pihole/pihole-password.cred
// ExecStart=/usr/local/bin/pihole-cli
// Restart=on-failure
// User=pihole-user

// # Security hardening
// DevicePolicy=closed
// ProtectSystem=strict
// ProtectHome=true
// PrivateTmp=true
// NoNewPrivileges=true

// [Install]
// WantedBy=multi-user.target
