use crate::level_manager::Level;

#[derive(Clone, Deserialize)]
pub struct LevelManagerSettings {
    pub levels: Vec<LevelSettings>,
}

#[derive(Clone, Deserialize)]
pub struct LevelSettings {
    pub level:    Level,
    pub filename: String,
    pub win_text: String,
}

impl LevelManagerSettings {
    pub fn level(&self, target: &Level) -> &LevelSettings {
        self.levels
            .iter()
            .find(|level| &level.level == target)
            .expect(&format!("Level {} should exist in settings", target))
    }
}
