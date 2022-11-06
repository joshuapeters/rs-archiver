use std::env::Args;

use crate::types::archive_strategy::ArchiveStrategy;

pub struct ArchiveArgs {
    pub dlc_path: String,
    pub archive_strategy: ArchiveStrategy
}

impl ArchiveArgs {
    pub fn parse_args(mut args: Args) -> ArchiveArgs {
        let dlc_path: String = args.nth(1).get_or_insert(String::new()).to_string().trim().to_owned();

        let archive_strategy: ArchiveStrategy = ArchiveStrategy::from_string(
            std::env::args()
                .nth(2)
                .get_or_insert(String::new())
                .to_string()
        );

        return ArchiveArgs {
            dlc_path,
            archive_strategy
        }
    }
}