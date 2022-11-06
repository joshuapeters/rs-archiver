#[derive(Debug)]
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

#[derive(Debug)]
struct ArchiveArgs {
    dlc_path: String,
    archive_strategy: ArchiveStrategy
}

fn main() {
    let dlc_path: String = std::env::args().nth(1).expect("please provide a path to your Rocksmith DLC directory.");

    if dlc_path.trim().is_empty() {
        eprintln!("path to DLC is required");
        return;
    }

    let archive_strategy: ArchiveStrategy = ArchiveStrategy::from_string(std::env::args().nth(2).expect("no archival strategy defined, defaulting to local"));

    let args = ArchiveArgs {
        dlc_path,
        archive_strategy
    };


    println!("{:?}", args);
}
