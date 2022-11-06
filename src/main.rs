use std::env::Args;

enum ArchiveStrategy {
    GoogleDocs,
    Local
}

impl ArchiveStrategy {
    fn from_string(mut strategy_as_string: String) -> ArchiveStrategy {
        strategy_as_string.make_ascii_lowercase();

        return match strategy_as_string.trim() {
            "google-docs" => ArchiveStrategy::GoogleDocs,
            _ => ArchiveStrategy::Local
        }
    }
}

struct ArchiveArgs {
    dlc_path: String,
    archive_strategy: ArchiveStrategy
}

impl ArchiveArgs {
    fn parse_args(mut args: Args) -> ArchiveArgs {
        let dlc_path: String = args.nth(1).expect("please provide an absolute path to your Rocksmith DLC directory.").to_string().trim().to_owned();

        let archive_strategy: ArchiveStrategy = ArchiveStrategy::from_string(std::env::args().nth(2).expect("no archival strategy defined, defaulting to local"));

        return ArchiveArgs {
            dlc_path,
            archive_strategy
        }
    }
}

fn main() {
    let args = ArchiveArgs::parse_args(std::env::args());

    if args.dlc_path.is_empty() {
        eprintln!("absolute path to DLC directory is required");
        return;
    }
}
