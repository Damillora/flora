use clap::{Args, Parser, Subcommand};
use flora::{
    app::{FloraAppOptions, FloraAppWineOptions}, errors::FloraError, manager::FloraManager
};

/// Manage your Wine and Proton prefixes
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates an app configuration
    Create(CreateArgs),
    /// Removes and app configuration
    Delete(DeleteArgs),
    /// Launches app configuration dialog
    Config(RunArgs),
    /// Launches wine(proton)tricks for app
    Tricks(RunArgs),
    /// Runs the app or another executable using the prefix
    Run(RunArgs),
    /// Creates a .desktop entry to launch the app from the application menu
    Desktop(DesktopArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    #[command(subcommand)]
    commands: CreateCommands,
}

#[derive(Subcommand)]
pub enum CreateCommands {
    /// Create a Wine app configuration,
    Wine(CreateWineArgs),
}

#[derive(Args)]
pub struct CreateAppArgs {
    /// Name of app
    name: String,

    /// Name on lists and desktop entries
    #[arg(short = 'n', long)]
    pretty_name: Option<String>,
    /// Default executable when launching app
    #[arg(short = 'e', long)]
    executable_location: Option<String>,
}

#[derive(Args)]
pub struct CreateWineArgs {
    #[command(flatten)]
    app: CreateAppArgs,

    /// Wine prefix for application
    #[arg(short = 'p', long)]
    wine_prefix: Option<String>,
    /// Wine runner for application
    #[arg(short = 'r', long)]
    wine_runner: Option<String>,
}

#[derive(Args)]
pub struct DeleteArgs {
    /// Name of app
    name: String,
}


#[derive(Args)]
pub struct RunArgs {
    /// Name of app
    name: String,
    /// Launch the specified executable
    args: Option<Vec<String>>,


    /// Redirect program output to flora logs
    #[arg(short, long)]
    quiet: bool,

    /// Wait until the app exits
    #[arg(short, long)]
    wait: bool,
}

#[derive(Args)]
pub struct DesktopArgs {
    /// Name of app
    name: String,
}


fn create_wine_app(manager: &FloraManager, args: &CreateWineArgs) -> Result<(), FloraError> {
    let app = FloraAppOptions::WineOptions(FloraAppWineOptions {
        pretty_name: args.app.pretty_name.clone(),
        executable_location: args.app.executable_location.clone().unwrap_or_default(),

        wine_prefix: args.wine_prefix.clone(),
        wine_runner: args.wine_runner.clone(),
    });

    manager.create_app(&args.app.name, &app)
}

fn main() -> Result<(), FloraError> {
    env_logger::init();
    let cli = Cli::parse();

    let manager = FloraManager::new();

    match &cli.command {
        Commands::Create(app_command) => match &app_command.commands {
            CreateCommands::Wine(args) => create_wine_app(&manager, args),
        },
        Commands::Delete(args) => manager.delete_app(&args.name),
        Commands::Config(args) => manager.app_config(&args.name, &args.args, args.quiet, args.wait),
        Commands::Tricks(args) => manager.app_tricks(&args.name, &args.args, args.quiet, args.wait),
        Commands::Run(args) =>  manager.app_run(&args.name, &args.args, args.quiet, args.wait),
        Commands::Desktop(args) => manager.create_desktop_entry(&args.name),
    }
}
