use crate::app::backend::resource_manager::LokResourceManager;
use crate::app::backend::Backend;
use crate::app::settings::Settings;

#[derive(Clone, Debug)]
pub struct PersistentData<BE: Backend> {
    pub lrm: LokResourceManager<BE>,
    settings: Settings,
}

impl<BE: Backend> PersistentData<BE> {
    pub async fn init_app_and_backend(db_url: &str) -> PersistentData<BE> {
        let lrm = LokResourceManager::<BE>::build(db_url)
            .await
            .expect("Couldn't create LokResourceManager");

        let settings = Settings::load().await;

        PersistentData {
            lrm,
            settings,
        }
    }

    pub fn get_lok_resource_manager(&self) -> LokResourceManager<BE> {
        self.lrm.clone()
    }

    pub fn get_settings(&self) -> Settings {
        self.settings.clone()
    }
}
