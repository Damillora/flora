// Create seed
pub enum FloraCreateSeed {
    WineOptions(FloraCreateWineSeed),
    ProtonOptions(FloraCreateProtonSeed),
}

pub enum FloraSeedAppOperations {
    Add(FloraCreateSeedApp),
    Update(FloraUpdateSeedApp),
    Rename(FloraRenameSeedApp),
    Delete(FloraDeleteSeedApp),
}

pub struct FloraCreateSeedApp {
    pub application_name: String,
    pub application_location: String,
}
pub struct FloraUpdateSeedApp {
    pub application_name: String,
    pub application_location: Option<String>,
}
pub struct FloraRenameSeedApp {
    pub old_application_name: String,
    pub new_application_name: String,
}
pub struct FloraDeleteSeedApp {
    pub application_name: String,
}

pub struct FloraCreateWineSeed {
    pub default_application_name: Option<String>,
    pub default_application_location: String,
    pub wine_prefix: Option<String>,
    pub wine_runner: Option<String>,
}

pub struct FloraCreateProtonSeed {
    pub default_application_name: Option<String>,
    pub default_application_location: String,
    pub proton_prefix: Option<String>,
    pub proton_runtime: Option<String>,
    pub game_id: Option<String>,
    pub store: Option<String>,
}

// Update
pub enum FloraUpdateSeed {
    WineOptions(FloraUpdateWineSeed),
    ProtonOptions(FloraUpdateProtonSeed),
}

pub struct FloraUpdateWineSeed {
    pub wine_prefix: Option<String>,
    pub wine_runtime: Option<String>,
}

pub struct FloraUpdateProtonSeed {
    pub proton_prefix: Option<String>,
    pub proton_runtime: Option<String>,
    pub game_id: Option<String>,
    pub store: Option<String>,
}
