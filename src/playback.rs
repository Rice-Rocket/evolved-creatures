pub enum PlaybackMode {
    Creature(usize),
    Generation(usize),
    Session,
}

pub struct PlaybackConfig {
    pub session: String,
    pub mode: PlaybackMode,
}

impl Default for PlaybackConfig {
    fn default() -> Self {
        Self { session: String::from("default-session"), mode: PlaybackMode::Creature(0) }
    }
}
