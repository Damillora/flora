use clap::{Args, Parser, Subcommand};
use flora::{
    errors::FloraError,
    manager::FloraManager,
    requests::{
        FloraCreateProtonSeed, FloraCreateSeed, FloraCreateSeedApp, FloraCreateSeedSettings,
        FloraCreateWineSeed, FloraDeleteSeedApp, FloraRenameSeedApp, FloraSeedAppOperations,
        FloraUpdateProtonSeed, FloraUpdateSeed, FloraUpdateSeedApp, FloraUpdateWineSeed,
    },
    responses::{FloraSeedAppItem, FloraSeedItem, FloraSeedStartMenuItem},
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
    /// Query Start Menu entries in a seed and create app entries based on them
    StartMenu(StartMenuOpts),
    /// Launch the seed's prefix configuration, usually winecfg.
    Config(RunOpts),
    /// Launch winetricks for the seed's prefix
    Tricks(RunOpts),
    /// Run an application in a seed
    Run(RunOpts),
    /// Generate menu entries for launching apps from the application menu
    GenerateMenu,
}

#[derive(Args)]
pub struct SeedOpts {
    #[command(subcommand)]
    commands: SeedCommands,
}

#[derive(Subcommand)]
pub enum SeedCommands {
    /// List all seeds
    List(ListOpts),
    /// Create a seed
    Create(CreateOpts),
    /// Set a seed's properties
    Set(SetOpts),
    /// Remove a seed
    Delete(DeleteOpts),
    /// Show a seed's information
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
    /// Launcher command for applications
    #[arg(short = 'c', long, required = false)]
    launcher: Option<String>,
}
#[derive(Args)]
#[group()]
pub struct CreateSeedDefaultOpts {
    /// Default application name for the seed
    #[arg(short = 'n', long, required = false)]
    app_name: String,
    /// Default executable location for the seed, passed to wine or proton.
    #[arg(short = 'l', long, required = false)]
    app_location: String,
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
    /// Add an app to a seed
    Add(AppAddOpts),
    /// Update an app in a seed
    Update(AppUpdateOpts),
    /// Rename an app in a seed
    Rename(AppRenameOpts),
    /// Remove an app from a seed
    Delete(AppDeleteOpts),
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
    app_name: String,
    /// Location for the app, passed to wine or proton.
    #[arg(short = 'l', long)]
    app_location: String,
}

#[derive(Args)]
pub struct AppUpdateOpts {
    #[clap(flatten)]
    seed: AppSeedOpts,

    /// Name for the app
    app_name: String,
    /// Location for the app, passed to wine or proton.
    #[arg(short = 'l', long)]
    app_location: Option<String>,
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
pub struct StartMenuOpts {
    #[clap(subcommand)]
    commands: StartMenuCommands,
}

#[derive(Subcommand)]
pub enum StartMenuCommands {
    /// List all Start Menu entries in a seed
    List(StartMenuListOpts),
    /// Generates an app based on a Start Menu entry
    CreateApp(StartMenuGenerateAppOpts),
}
#[derive(Args)]
pub struct StartMenuListOpts {
    /// Name of seed
    name: String,

    /// Long format
    #[arg(short = 'l', long)]
    long: bool,
}
#[derive(Args)]
pub struct StartMenuGenerateAppOpts {
    /// Name of seed
    name: String,

    /// Name of the Start Menu entry
    app_name: String,

    #[arg(short = 'c', long)]
    app_command: Option<String>,
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
pub struct SeedTableRow<'a> {
    pub name: &'a str,
    pub prefix: &'a str,
    pub runtime: &'a str,
    pub game_id: &'a str,
    pub store: &'a str,
}

#[derive(Tabled)]
#[tabled(rename_all = "Upper Title Case")]
pub struct SeedAppTableRow<'a> {
    pub application_name: &'a str,
    pub application_location: &'a str,
}

#[derive(Tabled)]
#[tabled(rename_all = "Upper Title Case")]
pub struct SeedStartMenuTableRow<'a> {
    pub name: &'a str,
    pub location: &'a str,
}

impl<'a> From<&'a FloraSeedItem> for SeedTableRow<'a> {
    fn from(item: &'a FloraSeedItem) -> Self {
        match &item.seed_type {
            flora::responses::FloraSeedTypeItem::Wine(conf) => Self {
                name: item.name.as_str(),
                prefix: conf.wine_prefix.as_deref().unwrap_or_default(),
                runtime: conf.wine_runtime.as_deref().unwrap_or_default(),
                game_id: "",
                store: "",
            },
            flora::responses::FloraSeedTypeItem::Proton(conf) => Self {
                name: item.name.as_str(),
                prefix: conf.proton_prefix.as_deref().unwrap_or_default(),
                runtime: conf.proton_runtime.as_deref().unwrap_or_default(),
                game_id: conf.game_id.as_deref().unwrap_or_default(),
                store: conf.store.as_deref().unwrap_or_default(),
            },
        }
    }
}
impl<'a> From<&'a FloraSeedAppItem> for SeedAppTableRow<'a> {
    fn from(item: &'a FloraSeedAppItem) -> Self {
        Self {
            application_name: item.application_name.as_str(),
            application_location: item.application_location.as_str(),
        }
    }
}
impl<'a> From<&'a FloraSeedStartMenuItem> for SeedStartMenuTableRow<'a> {
    fn from(item: &'a FloraSeedStartMenuItem) -> Self {
        Self {
            name: item.start_menu_name.as_str(),
            location: item.start_menu_location.as_str(),
        }
    }
}

fn create_wine_seed(manager: &FloraManager, args: &CreateWineOpts) -> Result<(), FloraError> {
    let seed = FloraCreateSeed::WineOptions(FloraCreateWineSeed {
        settings: Some(FloraCreateSeedSettings {
            launcher_command: args.seed.launcher.as_deref(),
        }),
        default_application: args
            .default_opts
            .as_ref()
            .map(|default_opt| FloraCreateSeedApp {
                application_name: default_opt.app_name.as_str(),
                application_location: default_opt.app_location.as_str(),
            }),

        wine_prefix: args.wine_prefix.as_deref(),
        wine_runner: args.wine_runtime.as_deref(),
    });

    manager.create_seed(&args.seed.name, &seed)
}

fn create_proton_seed(manager: &FloraManager, args: &CreateProtonOpts) -> Result<(), FloraError> {
    let seed = FloraCreateSeed::ProtonOptions(FloraCreateProtonSeed {
        settings: Some(FloraCreateSeedSettings {
            launcher_command: args.seed.launcher.as_deref(),
        }),
        default_application: args
            .default_opts
            .as_ref()
            .map(|default_opt| FloraCreateSeedApp {
                application_name: default_opt.app_name.as_str(),
                application_location: default_opt.app_location.as_str(),
            }),

        proton_prefix: args.proton_prefix.as_deref(),
        proton_runtime: args.proton_runtime.as_deref(),
        game_id: args.game_id.as_deref(),
        store: args.store.as_deref(),
    });

    manager.create_seed(&args.seed.name, &seed)
}

fn set_wine_seed(manager: &FloraManager, args: &SetWineOpts) -> Result<(), FloraError> {
    let seed_opts = FloraUpdateSeed::WineOptions(FloraUpdateWineSeed {
        wine_prefix: args.wine_prefix.as_deref(),
        wine_runtime: args.wine_runtime.as_deref(),
    });
    manager.update_seed(&args.seed.name, &seed_opts)?;
    Ok(())
}

fn set_proton_seed(manager: &FloraManager, args: &SetProtonOpts) -> Result<(), FloraError> {
    let seed_opts = FloraUpdateSeed::ProtonOptions(FloraUpdateProtonSeed {
        proton_prefix: args.proton_prefix.as_deref(),
        proton_runtime: args.proton_runtime.as_deref(),
        game_id: args.game_id.as_deref(),
        store: args.store.as_deref(),
    });

    manager.update_seed(&args.seed.name, &seed_opts)?;
    Ok(())
}
fn main() -> Result<(), FloraError> {
    env_logger::init();
    let cli = Cli::parse();

    let manager = FloraManager::new()?;

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
                    application_name: app_add_opts.app_name.as_str(),
                    application_location: app_add_opts.app_location.as_str(),
                })],
            ),
            AppCommands::Update(app_update_opts) => manager.update_seed_apps(
                &app_update_opts.seed.name,
                &vec![FloraSeedAppOperations::Update(FloraUpdateSeedApp {
                    application_name: app_update_opts.app_name.as_str(),
                    application_location: app_update_opts.app_location.as_deref(),
                })],
            ),
            AppCommands::Rename(app_rename_opts) => manager.update_seed_apps(
                &app_rename_opts.seed.name,
                &vec![FloraSeedAppOperations::Rename(FloraRenameSeedApp {
                    old_application_name: app_rename_opts.old_application_name.as_str(),
                    new_application_name: app_rename_opts.new_application_name.as_str(),
                })],
            ),
            AppCommands::Delete(app_delete_opts) => manager.update_seed_apps(
                &app_delete_opts.seed.name,
                &vec![FloraSeedAppOperations::Delete(FloraDeleteSeedApp {
                    application_name: app_delete_opts.application_name.as_str(),
                })],
            ),
        },
        Commands::StartMenu(opts) => match &opts.commands {
            StartMenuCommands::List(start_menu_list_opts) => {
                let start_menu_entries =
                    manager.list_start_menu_entries(&start_menu_list_opts.name)?;
                if start_menu_list_opts.long {
                    let table_items = start_menu_entries.iter().map(SeedStartMenuTableRow::from);

                    let mut table = Table::new(table_items);
                    table.with(Style::blank());
                    table.with(Colorization::exact([Color::FG_BRIGHT_BLUE], Rows::first()));
                    table.modify(Columns::first(), Alignment::left());

                    println!("{}", table);
                } else {
                    for entry in start_menu_entries {
                        println!("{}", entry.start_menu_name,)
                    }
                }

                Ok(())
            }
            StartMenuCommands::CreateApp(start_menu_create_app_opts) => manager
                .create_start_menu_app(
                    &start_menu_create_app_opts.name,
                    &start_menu_create_app_opts.app_name,
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
                        let joined_args = args.join(" ");
                        manager.seed_run_app(
                            &opts.name,
                            &Some(joined_args.as_str()),
                            opts.quiet,
                            opts.wait,
                        )
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
