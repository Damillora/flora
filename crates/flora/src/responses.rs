use crate::app::FloraApp;

pub struct FloraAppListItem {
    pub name: String,
    pub pretty_name: String,
    pub executable_location: String,

    pub app_type: FloraAppTypeListItem,
}

pub enum FloraAppTypeListItem {
    Wine(FloraAppWineListItem),
    Proton(FloraAppProtonListItem),
}

pub struct FloraAppWineListItem {
    pub wine_prefix: Option<String>,
    pub wine_runtime: Option<String>,
}

pub struct FloraAppProtonListItem {
    pub proton_prefix: Option<String>,
    pub proton_runtime: Option<String>,
    pub game_id: Option<String>,
    pub store: Option<String>,
}

impl FloraAppListItem {
    pub(crate) fn from_config(name: &String, config: &FloraApp) -> FloraAppListItem {
        FloraAppListItem {
            name: String::from(name),
            pretty_name: config.pretty_name.clone(),
            executable_location: config.executable_location.clone(),

            app_type: match &config.app_type {
                crate::app::FloraAppType::Wine(flora_app_wine_config) => {
                    FloraAppTypeListItem::Wine(FloraAppWineListItem {
                        wine_prefix: flora_app_wine_config.wine_prefix.clone(),
                        wine_runtime: flora_app_wine_config.wine_runtime.clone(),
                    })
                }
                crate::app::FloraAppType::Proton(flora_app_proton_config) => {
                    FloraAppTypeListItem::Proton(FloraAppProtonListItem {
                        proton_prefix: flora_app_proton_config.proton_prefix.clone(),
                        proton_runtime: flora_app_proton_config.proton_runtime.clone(),
                        game_id: flora_app_proton_config.game_id.clone(),
                        store: flora_app_proton_config.store.clone(),
                    })
                }
            },
        }
    }
}
