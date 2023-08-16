use flatgeobuf::{FgbWriter, GeometryType};
use walkdir::WalkDir;

use geozero::geojson::GeoJsonLineReader;
use geozero::GeozeroDatasource;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

mod chained_reader;
use chained_reader::ChainedReader;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let input_dir = Path::new(args.get(1).expect("required argument: <input dir>"));

    let input_paths = WalkDir::new(input_dir)
        .sort_by_file_name()
        .into_iter()
        .flat_map(|entry| {
            let entry = entry.expect("valid directory entry");
            if entry.metadata().expect("valid metadata").is_dir() {
                return None;
            }
            Some(entry.path().to_owned())
        })
        .collect();
    let mut input = ChainedReader::new(input_paths);

    let stem = input_dir.file_stem().expect("stemmable file name");

    let output_path = format!("{stem}.fgb", stem = stem.to_string_lossy());
    let output_file = File::create(output_path)?;
    let mut output = BufWriter::new(output_file);

    let mut reader = GeoJsonLineReader::new(&mut input);
    let mut writer = FgbWriter::create(&stem.to_string_lossy(), GeometryType::Point)?;

    eprintln!("start processing");
    reader.process(&mut writer)?;
    eprintln!("done processing");
    eprintln!("start writing");
    writer.write(&mut output)?;
    eprintln!("done writing");
    Ok(())
}
