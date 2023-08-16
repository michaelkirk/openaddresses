use flatgeobuf::{FgbWriter, GeometryType};

use geozero::geojson::GeoJsonLineReader;
use geozero::GeozeroDatasource;

use std::path::Path;
use std::io::BufWriter;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    let input_path = Path::new(args.get(1).expect("required argument: <input path>"));
    let input_file = std::fs::File::open(input_path)?;

    let stem = input_path.file_stem().expect("stemmable file name");
    let output_path = format!("{stem}.fgb", stem=stem.to_string_lossy());
    let output_file = std::fs::File::create(output_path)?;
    let mut output = BufWriter::new(output_file);

    let mut reader = GeoJsonLineReader::new(&input_file);
    let mut writer = FgbWriter::create(&stem.to_string_lossy(), GeometryType::Point)?;

    eprintln!("start processing");
    reader.process(&mut writer)?;
    eprintln!("done processing");
    eprintln!("start writing");
    writer.write(&mut output)?;
    eprintln!("done writing");
    Ok(())
}
