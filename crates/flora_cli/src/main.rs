use clap::{Args, Parser, Subcommand};
use flora::{
    errors::FloraError,
    manager::FloraManager,
    requests::{
        FloraCreateProtonSeed, FloraCreateSeed, FloraCreateWineSeed, FloraUpdateProtonSeed,
        FloraUpdateSeed, FloraUpdateWineSeed,
    },
    responses::{FloraSeedAppItem, FloraSeedItem},
};
use tabled::{
    Table, Tabled,
    settings::{
        Alignment, Color, Style,
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
    Create(CreateOpts),
    /// Set a seed's properties
    Set(SetOpts),
    /// Removes a seed
    Delete(DeleteOpts),
    /// Lists seeds
    List(ListOpts),
    /// Show a seed's configuration
    Show(ShowOpts),
    /// Launches a configuration tool inside the seed's prefix
    Config(RunOpts),
    /// Launches wine(proton)tricks for a seed's prefix
    Tricks(RunOpts),
    /// Runs an app or another executable in a seed
    Run(RunOpts),
    /// Creates a .desktop entry to launch applications inside a seed from the application menu
    Desktop(DesktopArgs),
}

#[derive(Args)]
pub struct CreateOpts {
    #[command(subcommand)]
    commands: CreateCommands,
}

#[derive(Subcommand)]
pub enum CreateCommands {
    /// Create a Wine seed
    Wine(CreateWineOpts),
    /// Create a Proton seed
    Proton(CreateProtonOpts),
}

#[derive(Args)]
pub struct CreateSeedOpts {
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
pub struct CreateWineOpts {
    #[command(flatten)]
    seed: CreateSeedOpts,

    /// Wine prefix for the seed
    #[arg(short = 'p', long)]
    wine_prefix: Option<String>,
    /// Wine runtime for the seed
    #[arg(short = 'r', long)]
    wine_runtime: Option<String>,
}
#[derive(Args)]
pub struct CreateProtonOpts {
    #[command(flatten)]
    seed: CreateSeedOpts,

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
pub struct SetOpts {
    #[command(subcommand)]
    commands: SetCommands,
}

#[derive(Subcommand)]
pub enum SetCommands {
    /// Set properties of a Wine seed
    Wine(SetWineOpts),
    /// Set propeties of a Proton seed
    Proton(SetProtonOpts),
}

#[derive(Args)]
pub struct SetSeedOpts {
    /// Name of seed
    name: String,
}
#[derive(Args)]
pub struct SetWineOpts {
    #[command(flatten)]
    seed: SetSeedOpts,

    /// Wine prefix for the seed
    #[arg(short = 'p', long)]
    wine_prefix: Option<String>,
    /// Wine runtime for the seed
    #[arg(short = 'r', long)]
    wine_runtime: Option<String>,
}

#[derive(Args)]
pub struct SetProtonOpts {
    #[command(flatten)]
    seed: SetSeedOpts,

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
pub struct DeleteOpts {
    /// Name of seed
    name: String,
}

#[derive(Args)]
pub struct ListOpts {
    #[arg(short = 'l', long)]
    /// Long format of list
    long: bool,
}
#[derive(Args)]
pub struct ShowOpts {
    /// Name of seed
    name: String,
}

#[derive(Args)]
pub struct RunOpts {
    /// Name of seed
    name: String,
    /// Launch the specified app
    args: Option<Vec<String>>,

    /// Run an app entry
    #[arg(short, long)]
    app: bool,
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
    pub prefix: String,
    pub runtime: String,
    pub game_id: String,
    pub store: String,
}

#[derive(Tabled)]
#[tabled(rename_all = "Upper Title Case")]
pub struct SeedAppTableRow {
    pub application_name: String,
    pub application_location: String,
}

impl From<&FloraSeedItem> for SeedTableRow {
    fn from(item: &FloraSeedItem) -> Self {
        match &item.seed_type {
            flora::responses::FloraSeedTypeItem::Wine(conf) => SeedTableRow {
                name: item.name.clone(),
                prefix: conf.wine_prefix.clone().unwrap_or_default(),
                runtime: conf.wine_runtime.clone().unwrap_or_default(),
                game_id: String::new(),
                store: String::new(),
            },
            flora::responses::FloraSeedTypeItem::Proton(conf) => SeedTableRow {
                name: item.name.clone(),
                prefix: conf.proton_prefix.clone().unwrap_or_default(),
                runtime: conf.proton_runtime.clone().unwrap_or_default(),
                game_id: conf.game_id.clone().unwrap_or_default(),
                store: conf.store.clone().unwrap_or_default(),
            },
        }
    }
}
impl From<&FloraSeedAppItem> for SeedAppTableRow {
    fn from(item: &FloraSeedAppItem) -> Self {
        SeedAppTableRow {
            application_name: item.application_name.clone(),
            application_location: item.application_location.clone(),
        }
    }
}

fn create_wine_seed(manager: &FloraManager, args: &CreateWineOpts) -> Result<(), FloraError> {
    let seed = FloraCreateSeed::WineOptions(FloraCreateWineSeed {
        default_application_name: args.seed.default_application_name.clone(),
        default_application_location: args
            .seed
            .default_executable_location
            .clone()
            .unwrap_or_default(),

        wine_prefix: args.wine_prefix.clone(),
        wine_runner: args.wine_runtime.clone(),
    });

    manager.create_seed(&args.seed.name, &seed)
}

fn create_proton_seed(manager: &FloraManager, args: &CreateProtonOpts) -> Result<(), FloraError> {
    let seed = FloraCreateSeed::ProtonOptions(FloraCreateProtonSeed {
        default_application_name: args.seed.default_application_name.clone(),
        default_application_location: args
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

fn set_wine_seed(manager: &FloraManager, args: &SetWineOpts) -> Result<(), FloraError> {
    let seed_opts = FloraUpdateSeed::WineOptions(FloraUpdateWineSeed {
        wine_prefix: args.wine_prefix.clone(),
        wine_runtime: args.wine_runtime.clone(),
    });
    manager.update_seed(&args.seed.name, &seed_opts)?;
    Ok(())
}

fn set_proton_seed(manager: &FloraManager, args: &SetProtonOpts) -> Result<(), FloraError> {
    let seed_opts = FloraUpdateSeed::ProtonOptions(FloraUpdateProtonSeed {
        proton_prefix: args.proton_prefix.clone(),
        proton_runtime: args.proton_runtime.clone(),
        game_id: args.game_id.clone(),
        store: args.store.clone(),
    });

    manager.update_seed(&args.seed.name, &seed_opts)?;
    Ok(())
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
        Commands::Set(create_opts) => match &create_opts.commands {
            SetCommands::Wine(args) => set_wine_seed(&manager, args),
            SetCommands::Proton(args) => set_proton_seed(&manager, args),
        },
        Commands::Delete(args) => manager.delete_seed(&args.name),
        Commands::List(args) => {
            let seeds = manager.list_seed()?;
            if args.long {
                let table_items = seeds.iter().map(SeedTableRow::from);

                let mut table = Table::new(table_items);
                table.with(Style::blank());
                table.with(Colorization::exact([Color::FG_BRIGHT_BLUE], Rows::first()));
                table.modify(Columns::first(), Alignment::left());

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
            let seed = manager.show_seed(&args.name)?;
            let seed_table = SeedTableRow::from(&seed);

            let mut table = Table::kv(vec![seed_table]);
            table.with(Style::blank());
            table.with(Colorization::exact(
                [Color::FG_BRIGHT_BLUE],
                Columns::first(),
            ));
            table.modify(Columns::first(), Alignment::left());

            println!("{}", table);
            println!("List of apps:");
            for app in seed.apps {
                let app_table = SeedAppTableRow::from(&app);
                let mut table = Table::kv(vec![app_table]);
                table.with(Style::blank());
                table.with(Colorization::exact(
                    [Color::FG_BRIGHT_BLUE],
                    Columns::first(),
                ));
                table.modify(Columns::first(), Alignment::left());
                println!("{}", table);
            }

            Ok(())
        }
        Commands::Config(args) => {
            manager.seed_config(&args.name, &args.args, args.quiet, args.wait)
        }
        Commands::Tricks(args) => {
            manager.seed_tricks(&args.name, &args.args, args.quiet, args.wait)
        }
        Commands::Run(opts) => {
            match &opts.args {
                Some(args) => {
                    if opts.app {
                        // Launch an app entry
                        let joined_args = args.join(" ").to_string();
                        manager.seed_run_app(&opts.name, &Some(joined_args), opts.quiet, opts.wait)
                    } else {
                        // Launch executable
                        manager.seed_run_executable(&opts.name, args, opts.quiet, opts.wait)
                    }
                }
                // Launch the default app entry if none is specified
                None => manager.seed_run_app(&opts.name, &None, opts.quiet, opts.wait),
            }
        }
        Commands::Desktop(args) => manager.create_desktop_entry(&args.name),
    }
}
