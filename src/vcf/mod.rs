use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use csv;
use flate2::read::MultiGzDecoder;

use self::reader::VcfReader;

pub mod reader;

static EMPTY_CELL: &str = ".:.";

pub fn split_file<P, Q>(
    src: P,
    dst: Q,
    disable_column: Option<usize>,
) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let file_reader = reader_factory(&src)?;

    let mut reader = VcfReader::new(file_reader);
    reader.read_meta()?;

    let mut writers = {
        let meta = reader.meta().unwrap();
        let headers = reader.mandatory_headers().unwrap();
        let samples: Vec<&str> = reader.samples().unwrap().iter()
            .enumerate()
            .filter(|(i, _)| disable_column.map(|j| *i != j).unwrap_or(true))
            .map(|(_, &id)| id)
            .collect();

        info!("{}: creating {} vcf(s)", src.as_ref().display(), samples.len());

        let mut writers: Vec<BufWriter<File>> = samples.iter()
            .map(|name| {
                let mut dst = dst.as_ref().to_path_buf();
                dst.push(format!("{}.vcf", name));
                let file = File::create(dst).unwrap();
                BufWriter::new(file)
            })
            .collect();

        for (writer, sample) in writers.iter_mut().zip(samples.iter()) {
            write!(writer, "{}", meta)?;

            for header in &headers {
                write!(writer, "{}\t", header)?;
            }

            writeln!(writer, "{}", sample)?;
        }

        writers
    };

    let n_headers = reader.n_headers();

    let mut csv = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_reader(reader.into_inner());

    for record in csv.records().filter_map(Result::ok) {
        let iter = record.iter()
            .skip(n_headers)
            .enumerate()
            .filter(|(i, _)| disable_column.map(|j| *i != j).unwrap_or(true))
            .map(|(_, value)| value)
            .enumerate();

        for (i, value) in iter {
            if value == EMPTY_CELL {
                continue;
            }

            let line = record.iter()
                .take(n_headers)
                .collect::<Vec<&str>>()
                .join("\t");

            writeln!(&mut writers[i], "{}\t{}", line, value)?;
        }
    }

    Ok(())
}

fn reader_factory<P>(src: P) -> io::Result<Box<dyn BufRead>> where P: AsRef<Path> {
    let path = src.as_ref();
    let file = File::open(path)?;

    match path.extension().and_then(OsStr::to_str) {
        Some("gz") => {
            let decoder = MultiGzDecoder::new(file);
            let reader = BufReader::new(decoder);
            Ok(Box::new(reader))
        },
        _ => {
            Ok(Box::new(BufReader::new(file)))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_split_file() {
        let dst = env::temp_dir();

        let result = split_file("test/fixtures/sample.single.vcf", &dst, None);
        assert!(result.is_ok());

        let result = split_file("test/fixtures/sample.multi.vcf", &dst, None);
        assert!(result.is_ok());
    }
}
