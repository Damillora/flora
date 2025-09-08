use clap::{Args, Parser, Subcommand};
use flora::{
    errors::FloraError,
    manager::FloraManager,
    responses::FloraSeedItem,
    seed::{FloraCreateProtonSeed, FloraCreateSeed, FloraCreateWineSeed},
};
use tabled::{
    Table, Tabled,
    settings::{
        Alignment, Color, Style, Width,
        object::{Columns, Rows},
        themes::Colorization,
    },
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
    /// Creates a seed
    Create(CreateArgs),
    /// Removes a seed
    Delete(DeleteArgs),
    /// Lists seeds
    List(ListArgs),
    /// Show a seed's configuration
    Show(ShowArgs),
    /// Launches a configuration tool inside the seed's prefix
    Config(RunArgs),
    /// Launches wine(proton)tricks for a seed's prefix
    Tricks(RunArgs),
    /// Runs an app or another executable in a seed
    Run(RunArgs),
    /// Creates a .desktop entry to launch applications inside a seed from the application menu
    Desktop(DesktopArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    #[command(subcommand)]
    commands: CreateCommands,
}

#[derive(Subcommand)]
pub enum CreateCommands {
    /// Create a Wine seed
    Wine(CreateWineArgs),
    /// Create a Proton seed
    Proton(CreateProtonArgs),
}

#[derive(Args)]
pub struct CreateSeedArgs {
    /// Name of seed
    name: String,

    /// Default application name for the seed
    #[arg(short = 'n', long)]
    default_application_name: Option<String>,
    /// Default executable location for the seed, passed to wine or proton.
    #[arg(short = 'e', long)]
    default_executable_location: Option<String>,
}

#[derive(Args)]
pub struct CreateWineArgs {
    #[command(flatten)]
    seed: CreateSeedArgs,

    /// Wine prefix for the seed
    #[arg(short = 'p', long)]
    wine_prefix: Option<String>,
    /// Wine runtime for the seed
    #[arg(short = 'r', long)]
    wine_runtime: Option<String>,
}
#[derive(Args)]
pub struct CreateProtonArgs {
    #[command(flatten)]
    seed: CreateSeedArgs,

    /// Proton prefix for the seed
    #[arg(short = 'p', long)]
    proton_prefix: Option<String>,
    /// Proton runtime for the seed
    #[arg(short = 'r', long)]
    proton_runtime: Option<String>,
    #[arg(short = 'g', long)]
    /// Game ID to be passed to umu-launcher
    game_id: Option<String>,
    #[arg(short = 's', long)]
    /// Store to be passed to umu-launcher
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
    /// Name of seed
    name: String,
}

#[derive(Args)]
pub struct DeleteArgs {
    /// Name of seed
    name: String,
}

#[derive(Args)]
pub struct RunArgs {
    /// Name of seed
    name: String,
    /// Launch the specified app
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
    /// Name of seed
    name: String,
}

#[derive(Tabled)]
#[tabled(rename_all = "Upper Title Case")]
pub struct SeedTableRow {
    pub name: String,
    pub pretty_name: String,
    pub executable_location: String,
    pub prefix: String,
    pub runtime: String,
    pub game_id: String,
    pub store: String,
}

impl From<&FloraSeedItem> for SeedTableRow {
    fn from(item: &FloraSeedItem) -> Self {
        match &item.seed_type {
            flora::responses::FloraSeedTypeItem::Wine(conf) => SeedTableRow {
                name: item.name.clone(),
                pretty_name: item.pretty_name.clone(),
                executable_location: item.executable_location.clone(),
                prefix: conf.wine_prefix.clone().unwrap_or_default(),
                runtime: conf.wine_runtime.clone().unwrap_or_default(),
                game_id: String::new(),
                store: String::new(),
            },
            flora::responses::FloraSeedTypeItem::Proton(conf) => SeedTableRow {
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

fn create_wine_seed(manager: &FloraManager, args: &CreateWineArgs) -> Result<(), FloraError> {
    let seed = FloraCreateSeed::WineOptions(FloraCreateWineSeed {
        pretty_name: args.seed.default_application_name.clone(),
        executable_location: args
            .seed
            .default_executable_location
            .clone()
            .unwrap_or_default(),

        wine_prefix: args.wine_prefix.clone(),
        wine_runner: args.wine_runtime.clone(),
    });

    manager.create_seed(&args.seed.name, &seed)
}

fn create_proton_seed(manager: &FloraManager, args: &CreateProtonArgs) -> Result<(), FloraError> {
    let seed = FloraCreateSeed::ProtonOptions(FloraCreateProtonSeed {
        pretty_name: args.seed.default_application_name.clone(),
        executable_location: args
            .seed
            .default_executable_location
            .clone()
            .unwrap_or_default(),

        proton_prefix: args.proton_prefix.clone(),
        proton_runtime: args.proton_runtime.clone(),
        game_id: args.game_id.clone(),
        store: args.store.clone(),
    });

    manager.create_seed(&args.seed.name, &seed)
}
fn main() -> Result<(), FloraError> {
    env_logger::init();
    let cli = Cli::parse();

    let manager = FloraManager::new();

    match &cli.command {
        Commands::Create(create_opts) => match &create_opts.commands {
            CreateCommands::Wine(args) => create_wine_seed(&manager, args),
            CreateCommands::Proton(args) => create_proton_seed(&manager, args),
        },
        Commands::Delete(args) => manager.delete_seed(&args.name),
        Commands::List(args) => {
            let seeds = manager.list_seed()?;
            if args.long {
                let table_items = seeds.iter().map(|item| SeedTableRow::from(item));

                let mut table = Table::new(table_items);
                table.with(Style::blank());
                table.with(Colorization::exact([Color::FG_BRIGHT_BLUE], Rows::first()));
                table.modify(Columns::first(), Alignment::left());
                table.modify(Rows::new(0..), Width::truncate(30).suffix("..."));

                println!("{}", table);
            } else {
                for seed in seeds {
                    println!(
                        "{} ({})",
                        seed.name,
                        match seed.seed_type {
                            flora::responses::FloraSeedTypeItem::Wine(_) => "Wine",
                            flora::responses::FloraSeedTypeItem::Proton(_) => "Proton",
                        }
                    )
                }
            }
            Ok(())
        }

        Commands::Show(args) => {
            let seed = SeedTableRow::from(&manager.show_seed(&args.name)?);

            let mut table = Table::kv(vec![seed]);
            table.with(Style::blank());
            table.with(Colorization::exact(
                [Color::FG_BRIGHT_BLUE],
                Columns::first(),
            ));
            table.modify(Columns::first(), Alignment::left());

            println!("{}", table);

            Ok(())
        }
        Commands::Config(args) => {
            manager.seed_config(&args.name, &args.args, args.quiet, args.wait)
        }
        Commands::Tricks(args) => {
            manager.seed_tricks(&args.name, &args.args, args.quiet, args.wait)
        }
        Commands::Run(args) => manager.seed_run(&args.name, &args.args, args.quiet, args.wait),
        Commands::Desktop(args) => manager.create_desktop_entry(&args.name),
    }
}
