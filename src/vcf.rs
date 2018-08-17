use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use csv;
use flate2::read::MultiGzDecoder;

const BUF_SIZE: usize = 4096; // bytes
static EMPTY_CELL: &str = ".:.";

pub fn split_file<P, Q>(src: P, dst: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let mut reader = reader_factory(&src)?;

    let mut line = String::with_capacity(BUF_SIZE);
    let mut meta = String::with_capacity(BUF_SIZE);

    let samples: Vec<&str>;

    loop {
        reader.read_line(&mut line)?;

        if line.starts_with("##") {
            meta.push_str(&line);
        } else if line.starts_with("#CHROM") {
            meta.push_str(&line[..45]);
            let line = line.trim();
            samples = line.split('\t').skip(9).collect();
            break;
        }

        line.clear();
    }

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
        writeln!(writer, "{}\t{}", meta, sample)?;
    }

    let mut csv = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_reader(reader);

    for record in csv.records().filter_map(Result::ok) {
        for (i, value) in record.iter().skip(9).enumerate() {
            if value == EMPTY_CELL {
                continue;
            }

            let line = record.iter()
                .take(9)
                .map(String::from)
                .collect::<Vec<String>>()
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
