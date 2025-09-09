// Create seed
pub enum FloraCreateSeed<'a> {
    WineOptions(FloraCreateWineSeed<'a>),
    ProtonOptions(FloraCreateProtonSeed<'a>),
}

pub enum FloraSeedAppOperations<'a> {
    Add(FloraCreateSeedApp<'a>),
    Update(FloraUpdateSeedApp<'a>),
    Rename(FloraRenameSeedApp<'a>),
    Delete(FloraDeleteSeedApp<'a>),
}

pub struct FloraCreateSeedApp<'a> {
    pub application_name: &'a str,
    pub application_location: &'a str,
}
pub struct FloraUpdateSeedApp<'a> {
    pub application_name: &'a str,
    pub application_location: Option<&'a str>,
}
pub struct FloraRenameSeedApp<'a> {
    pub old_application_name: &'a str,
    pub new_application_name: &'a str,
}
pub struct FloraDeleteSeedApp<'a> {
    pub application_name: &'a str,
}

pub struct FloraCreateWineSeed<'a> {
    pub default_application: Option<FloraCreateSeedApp<'a>>,
    pub wine_prefix: Option<&'a str>,
    pub wine_runner: Option<&'a str>,
}

pub struct FloraCreateProtonSeed<'a> {
    pub default_application: Option<FloraCreateSeedApp<'a>>,
    pub proton_prefix: Option<&'a str>,
    pub proton_runtime: Option<&'a str>,
    pub game_id: Option<&'a str>,
    pub store: Option<&'a str>,
}

// Update
pub enum FloraUpdateSeed<'a> {
    WineOptions(FloraUpdateWineSeed<'a>),
    ProtonOptions(FloraUpdateProtonSeed<'a>),
}

pub struct FloraUpdateWineSeed<'a> {
    pub wine_prefix: Option<&'a str>,
    pub wine_runtime: Option<&'a str>,
}

pub struct FloraUpdateProtonSeed<'a> {
    pub proton_prefix: Option<&'a str>,
    pub proton_runtime: Option<&'a str>,
    pub game_id: Option<&'a str>,
    pub store: Option<&'a str>,
}
