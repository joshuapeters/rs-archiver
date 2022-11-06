pub mod types;

use std::{path::Path, fs::File, io::{Read, Write}};
use zip::{result::ZipError, write::FileOptions};
use walkdir::{DirEntry, WalkDir};

use crate::types::archive_args::ArchiveArgs;

fn main() {
    let args = ArchiveArgs::parse_args(std::env::args());

    if args.dlc_path.is_empty() {
        eprintln!("absolute path to DLC directory is required");
        return;
    }

    // create zip of dlc directory with run time as name
    let _zip_result = zip(&args);

    // get archiver for provided strategy

    // archive
}

// 
fn zip(archive_args: &ArchiveArgs) -> zip::result::ZipResult<Option<&str>> {
    if !Path::new(&archive_args.dlc_path).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new("archive.zip");
    let file = File::create(&path).unwrap();

    let directory_iterator = WalkDir::new(&archive_args.dlc_path).into_iter();

    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Bzip2).large_file(true);

    let mut buffer = Vec::new();

    for entry in directory_iterator {
        let path = entry.unwrap().path().to_owned();
        let name = path.strip_prefix(Path::new(&archive_args.dlc_path)).unwrap();

        // Write file
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("name is {:?}", name);
            println!("extension is {:?}", name.extension().unwrap());

            // psarc is the file extension of rocksmith dlc track files
            if name.extension().unwrap() != "psarc" {
                continue;
            }

            println!("adding file {:?} as {:?} ...", path, name);

            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;

            buffer.clear();
        }
    }

    zip.finish()?;

    return Result::Ok(path.to_str());
}
