use clap::{Args, Parser, Subcommand};
use flora::{
    errors::FloraError,
    manager::FloraManager,
    requests::{
        FloraCreateProtonSeed, FloraCreateSeed, FloraCreateSeedApp, FloraCreateWineSeed,
        FloraDeleteSeedApp, FloraRenameSeedApp, FloraSeedAppOperations, FloraUpdateProtonSeed,
        FloraUpdateSeed, FloraUpdateSeedApp, FloraUpdateWineSeed,
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
    /// Manage seeds
    Seed(SeedOpts),
    /// Manage apps in a seed
    App(AppOpts),
    /// Launches the seed's prefix configuration, usually winecfg.
    Config(RunOpts),
    /// Launches winetricks for the seed's prefix
    Tricks(RunOpts),
    /// Runs an application in a seed
    Run(RunOpts),
    /// Generates menu entries for each app entries in a seed for launching from the application menu
    GenerateMenu,
}

#[derive(Args)]
pub struct SeedOpts {
    #[command(subcommand)]
    commands: SeedCommands,
}

#[derive(Subcommand)]
pub enum SeedCommands {
    /// Lists all seeds
    List(ListOpts),
    /// Creates a seed
    Create(CreateOpts),
    /// Set a seed's properties
    Set(SetOpts),
    /// Removes a seed
    Delete(DeleteOpts),
    /// Shows a seed's information
    Info(InfoOpts),
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
}
#[derive(Args)]
#[group()]
pub struct CreateSeedDefaultOpts {
    /// Default application name for the seed
    #[arg(short = 'n', long, required = false)]
    default_application_name: String,
    /// Default executable location for the seed, passed to wine or proton.
    #[arg(short = 'l', long, required = false)]
    default_application_location: String,
}
#[derive(Args)]
pub struct CreateWineOpts {
    #[command(flatten)]
    seed: CreateSeedOpts,

    #[clap(flatten)]
    default_opts: Option<CreateSeedDefaultOpts>,
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

    #[clap(flatten)]
    default_opts: Option<CreateSeedDefaultOpts>,

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
pub struct InfoOpts {
    /// Name of seed
    name: String,
}

#[derive(Args)]
pub struct AppOpts {
    #[command(subcommand)]
    commands: AppCommands,
}

#[derive(Subcommand)]
pub enum AppCommands {
    /// List all apps in a seed
    List(AppListOpts),
    /// Adds an app to a seed
    Add(AppAddOpts),
    /// Updates an app in a seed
    Update(AppUpdateOpts),
    /// Renames an app in a seed
    Rename(AppRenameOpts),
    /// Removes an app from a seed
    Delete(AppDeleteOpts),
    /// Generates an app entry from a Start Menu shortcut
    StartMenu(AppStartMenuOpts),
}
#[derive(Args)]
pub struct AppSeedOpts {
    /// Name of seed
    name: String,
}
#[derive(Args)]
pub struct AppListOpts {
    #[clap(flatten)]
    seed: AppSeedOpts,

    /// Long format
    #[arg(short = 'l', long)]
    long: bool,
}

#[derive(Args)]
pub struct AppAddOpts {
    #[clap(flatten)]
    seed: AppSeedOpts,

    /// Name for the app
    application_name: String,
    /// Location for the app, passed to wine or proton.
    #[arg(short = 'l', long)]
    application_location: String,
}

#[derive(Args)]
pub struct AppUpdateOpts {
    #[clap(flatten)]
    seed: AppSeedOpts,

    /// Name for the app
    application_name: String,
    /// Location for the app, passed to wine or proton.
    #[arg(short = 'l', long)]
    application_location: Option<String>,
}
#[derive(Args)]
pub struct AppRenameOpts {
    #[clap(flatten)]
    seed: AppSeedOpts,

    /// Old name of the app
    old_application_name: String,
    /// New name of the app
    new_application_name: String,
}

#[derive(Args)]
pub struct AppDeleteOpts {
    #[clap(flatten)]
    seed: AppSeedOpts,

    /// Name for the app
    application_name: String,
}
#[derive(Args)]
pub struct AppStartMenuOpts {
    #[clap(flatten)]
    seed: AppSeedOpts,

    /// Name of the Start Menu entry
    application_name: String,
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
        default_application: args
            .default_opts
            .as_ref()
            .map(|default_opt| FloraCreateSeedApp {
                application_name: default_opt.default_application_name.clone(),
                application_location: default_opt.default_application_location.clone(),
            }),

        wine_prefix: args.wine_prefix.clone(),
        wine_runner: args.wine_runtime.clone(),
    });

    manager.create_seed(&args.seed.name, &seed)
}

fn create_proton_seed(manager: &FloraManager, args: &CreateProtonOpts) -> Result<(), FloraError> {
    let seed = FloraCreateSeed::ProtonOptions(FloraCreateProtonSeed {
        default_application: args
            .default_opts
            .as_ref()
            .map(|default_opt| FloraCreateSeedApp {
                application_name: default_opt.default_application_name.clone(),
                application_location: default_opt.default_application_location.clone(),
            }),

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
        Commands::Seed(opts) => match &opts.commands {
            SeedCommands::Create(create_opts) => match &create_opts.commands {
                CreateCommands::Wine(args) => create_wine_seed(&manager, args),
                CreateCommands::Proton(args) => create_proton_seed(&manager, args),
            },
            SeedCommands::Set(create_opts) => match &create_opts.commands {
                SetCommands::Wine(args) => set_wine_seed(&manager, args),
                SetCommands::Proton(args) => set_proton_seed(&manager, args),
            },
            SeedCommands::Delete(args) => manager.delete_seed(&args.name),
            SeedCommands::List(args) => {
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
            SeedCommands::Info(args) => {
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
        },
        Commands::App(app_opts) => match &app_opts.commands {
            AppCommands::List(app_list_opts) => {
                let seed = manager.show_seed(&app_list_opts.seed.name)?;
                if app_list_opts.long {
                    let table_items = seed.apps.iter().map(SeedAppTableRow::from);

                    let mut table = Table::new(table_items);
                    table.with(Style::blank());
                    table.with(Colorization::exact([Color::FG_BRIGHT_BLUE], Rows::first()));
                    table.modify(Columns::first(), Alignment::left());

                    println!("{}", table);
                } else {
                    for app in seed.apps {
                        println!("{}", app.application_name,)
                    }
                }
                Ok(())
            }
            AppCommands::Add(app_add_opts) => manager.update_seed_apps(
                &app_add_opts.seed.name,
                &vec![FloraSeedAppOperations::Add(FloraCreateSeedApp {
                    application_name: app_add_opts.application_name.clone(),
                    application_location: app_add_opts.application_location.clone(),
                })],
            ),
            AppCommands::Update(app_update_opts) => manager.update_seed_apps(
                &app_update_opts.seed.name,
                &vec![FloraSeedAppOperations::Update(FloraUpdateSeedApp {
                    application_name: app_update_opts.application_name.clone(),
                    application_location: app_update_opts.application_location.clone(),
                })],
            ),
            AppCommands::Rename(app_rename_opts) => manager.update_seed_apps(
                &app_rename_opts.seed.name,
                &vec![FloraSeedAppOperations::Rename(FloraRenameSeedApp {
                    old_application_name: app_rename_opts.old_application_name.clone(),
                    new_application_name: app_rename_opts.new_application_name.clone(),
                })],
            ),
            AppCommands::Delete(app_delete_opts) => manager.update_seed_apps(
                &app_delete_opts.seed.name,
                &vec![FloraSeedAppOperations::Delete(FloraDeleteSeedApp {
                    application_name: app_delete_opts.application_name.clone(),
                })],
            ),
            AppCommands::StartMenu(app_start_menu_opts) => manager.create_start_menu_app(
                &app_start_menu_opts.seed.name,
                &app_start_menu_opts.application_name,
            ),
        },
        Commands::Config(opts) => {
            let args = opts
                .args
                .as_ref()
                .map(|m| m.iter().map(|s| s.as_str()).collect());
            manager.seed_config(&opts.name, &args, opts.quiet, opts.wait)
        }
        Commands::Tricks(opts) => {
            let args = opts
                .args
                .as_ref()
                .map(|m| m.iter().map(|s| s.as_str()).collect());
            manager.seed_tricks(&opts.name, &args, opts.quiet, opts.wait)
        }
        Commands::Run(opts) => {
            match &opts.args {
                Some(args) => {
                    if opts.app {
                        // Launch an app entry
                        let joined_args = args.join(" ").to_string();
                        manager.seed_run_app(&opts.name, &Some(joined_args), opts.quiet, opts.wait)
                    } else {
                        let args = args.iter().map(|s| s.as_str()).collect();
                        // Launch executable
                        manager.seed_run_executable(&opts.name, &args, opts.quiet, opts.wait)
                    }
                }
                // Launch the default app entry if none is specified
                None => manager.seed_run_app(&opts.name, &None, opts.quiet, opts.wait),
            }
        }
        Commands::GenerateMenu => manager.create_desktop_entry(),
    }
}
