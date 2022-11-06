pub enum ArchiveStrategy {
    GoogleDocs,
    Local
}

impl ArchiveStrategy {
    pub fn from_string(mut strategy_as_string: String) -> ArchiveStrategy {
        strategy_as_string = strategy_as_string.to_ascii_lowercase().trim().to_owned();



        if strategy_as_string.is_empty() {
            println!("No strategy found, defaulting to local file");
        }

        return match strategy_as_string.as_str() {
            "google-docs" => ArchiveStrategy::GoogleDocs,
            _ => ArchiveStrategy::Local
        }
    }
}