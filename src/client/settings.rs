pub struct Settings {
    pub resolution: (u32, u32),
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            resolution: (800, 600),
        }
    }
}
