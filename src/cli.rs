use clap::{Arg, Command};
use crate::auth::AuthService;

pub fn cli() -> Command {
    Command::new("your-app")
        .subcommand(
            Command::new("create-admin-key")
                .about("Create an initial admin API key")
                .arg(
                    Arg::new("key")
                        .long("key")
                        .help("Custom key (optional, will generate if not provided)")
                        .value_name("KEY")
                )
        )
}

pub fn handle_cli(auth_service: AuthService) -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("create-admin-key", sub_matches)) => {
            let key = match sub_matches.get_one::<String>("key") {
                Some(custom_key) => custom_key.clone(),
                None => AuthService::generate_api_key(),
            };

            match auth_service.create_api_key(&key, true) {
                Ok(api_key) => {
                    println!("api key created successfully!");
                    println!("Key: {}", key);
                    println!("ID: {}", api_key.id);
                    println!("Created at: {}", api_key.created_at);
                }
                Err(e) => {
                    eprintln!("failed to create new admin key: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            cli().print_help()?;
        }
    }

    Ok(())
}
