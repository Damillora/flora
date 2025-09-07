use clap::{Args, Parser, Subcommand};
use flora::{
    app::{FloraAppOptions, FloraAppProtonOptions, FloraAppWineOptions},
    errors::FloraError,
    manager::FloraManager, responses::FloraAppListItem,
};
use tabled::{settings::{object::{Columns, Rows}, themes::Colorization, Alignment, Color, Style, Width}, Table, Tabled};

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
    /// Lists app configurations
    List(ListArgs),
    /// Show app configuration
    Show(ShowArgs),
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
    /// Create a Proton app configuration,
    Proton(CreateProtonArgs),
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
pub struct CreateProtonArgs {
    #[command(flatten)]
    app: CreateAppArgs,

    /// Proton prefix for application
    #[arg(short = 'p', long)]
    proton_prefix: Option<String>,
    /// Proton runtime for application
    #[arg(short = 'r', long)]
    proton_runtime: Option<String>,
    #[arg(short = 'g', long)]
    /// Game ID to be passed to UMU
    game_id: Option<String>,
    #[arg(short = 's', long)]
    /// Store to be passed to UMU
    store: Option<String>,
}

#[derive(Args)]
pub struct ListArgs {
    #[arg(short = 'l', long)]
    /// Long format of list
    long: bool,
}
#[derive(Args)]
pub struct ShowArgs {
    /// Name of app
    name: String,
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

#[derive(Tabled)]
#[tabled(rename_all = "Upper Title Case")]
pub struct AppTableRow {
    pub name: String,
    pub pretty_name: String,
    pub executable_location: String,
    pub prefix: String,
    pub runtime: String,
    pub game_id: String,
    pub store: String,
}

impl From<&FloraAppListItem> for AppTableRow {
    fn from(item: &FloraAppListItem) -> Self {
        match &item.app_type {
            flora::responses::FloraAppTypeListItem::Wine(conf) => AppTableRow {
                name: item.name.clone(),
                pretty_name: item.pretty_name.clone(),
                executable_location: item.executable_location.clone(),
                prefix: conf.wine_prefix.clone().unwrap_or_default(),
                runtime: conf.wine_runtime.clone().unwrap_or_default(),
                game_id: String::new(),
                store: String::new(),
            },
            flora::responses::FloraAppTypeListItem::Proton(conf) => AppTableRow {
                name: item.name.clone(),
                pretty_name: item.pretty_name.clone(),
                executable_location: item.executable_location.clone(),
                prefix: conf.proton_prefix.clone().unwrap_or_default(),
                runtime: conf.proton_runtime.clone().unwrap_or_default(),
                game_id: conf.game_id.clone().unwrap_or_default(),
                store: conf.store.clone().unwrap_or_default(),
            },
        }
    }
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

fn create_proton_app(manager: &FloraManager, args: &CreateProtonArgs) -> Result<(), FloraError> {
    let app = FloraAppOptions::ProtonOptions(FloraAppProtonOptions {
        pretty_name: args.app.pretty_name.clone(),
        executable_location: args.app.executable_location.clone().unwrap_or_default(),

        proton_prefix: args.proton_prefix.clone(),
        proton_runtime: args.proton_runtime.clone(),
        game_id: args.game_id.clone(),
        store: args.store.clone(),
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
            CreateCommands::Proton(args) => create_proton_app(&manager, args),
        },
        Commands::Delete(args) => manager.delete_app(&args.name),
        Commands::List(args) => {
            let apps = manager.list_app()?;
            if args.long {
                let table_items = apps.iter().map(|item| AppTableRow::from(item));



                let mut table = Table::new(table_items);
                table.with(Style::blank());
                table.with(Colorization::exact([Color::FG_BRIGHT_BLUE], Rows::first() ));
                table.modify(Columns::first(), Alignment::left());
                table.modify(Rows::new(0..), Width::truncate(30).suffix("...") );

                println!("{}", table);
            } else {
                for app in apps {
                    println!(
                        "{} ({})",
                        app.name,
                        match app.app_type {
                            flora::responses::FloraAppTypeListItem::Wine(_) => "Wine",
                            flora::responses::FloraAppTypeListItem::Proton(_) => "Proton",
                        }
                    )
                }
            }
            Ok(())
        },

        Commands::Show(args) => {
            let app = AppTableRow::from(&manager.show_app(&args.name)?);

            let mut table = Table::kv(vec!(app));
            table.with(Style::blank());
            table.with(Colorization::exact([Color::FG_BRIGHT_BLUE], Columns::first() ));
            table.modify(Columns::first(), Alignment::left());

            println!("{}", table);

            Ok(())
        }
        Commands::Config(args) => manager.app_config(&args.name, &args.args, args.quiet, args.wait),
        Commands::Tricks(args) => manager.app_tricks(&args.name, &args.args, args.quiet, args.wait),
        Commands::Run(args) => manager.app_run(&args.name, &args.args, args.quiet, args.wait),
        Commands::Desktop(args) => manager.create_desktop_entry(&args.name),
    }
}
