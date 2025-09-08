use crate::seed::FloraSeed;

pub struct FloraSeedItem {
    pub name: String,
    pub pretty_name: String,
    pub executable_location: String,

    pub seed_type: FloraSeedTypeItem,
}

pub enum FloraSeedTypeItem {
    Wine(FloraWineSeedItem),
    Proton(FloraProtonSeedItem),
}

pub struct FloraWineSeedItem {
    pub wine_prefix: Option<String>,
    pub wine_runtime: Option<String>,
}

pub struct FloraProtonSeedItem {
    pub proton_prefix: Option<String>,
    pub proton_runtime: Option<String>,
    pub game_id: Option<String>,
    pub store: Option<String>,
}

impl FloraSeedItem {
    pub(crate) fn from_config(name: &String, config: &FloraSeed) -> FloraSeedItem {
        FloraSeedItem {
            name: String::from(name),
            pretty_name: config.pretty_name.clone(),
            executable_location: config.executable_location.clone(),

            seed_type: match &config.seed_type {
                crate::seed::FloraSeedType::Wine(flora_wine_seed) => {
                    FloraSeedTypeItem::Wine(FloraWineSeedItem {
                        wine_prefix: flora_wine_seed.wine_prefix.clone(),
                        wine_runtime: flora_wine_seed.wine_runtime.clone(),
                    })
                }
                crate::seed::FloraSeedType::Proton(flora_proton_seed) => {
                    FloraSeedTypeItem::Proton(FloraProtonSeedItem {
                        proton_prefix: flora_proton_seed.proton_prefix.clone(),
                        proton_runtime: flora_proton_seed.proton_runtime.clone(),
                        game_id: flora_proton_seed.game_id.clone(),
                        store: flora_proton_seed.store.clone(),
                    })
                }
            },
        }
    }
}
