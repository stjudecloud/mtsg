use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const DEFAULT_BUF_SIZE: usize = 8 * 1024; // bytes

static META_COMMENT: &str = "##";
static FILE_FORMAT: &str = "##fileformat=VCF";
static MANDATORY_HEADERS: &[&str] = &[
    "#CHROM", "POS", "ID", "REF", "ALT", "QUAL", "FILTER", "INFO",
];
static OPTIONAL_HEADER: &str = "FORMAT";

pub struct VcfReader<R: BufRead> {
    inner: R,
    line_no: usize,
    meta: Option<String>,
    headers: Option<String>,
    has_format: bool,
}

impl<R: BufRead> VcfReader<R> {
    pub fn open<P>(src: P) -> io::Result<VcfReader<BufReader<File>>> where P: AsRef<Path> {
        let file = File::open(src)?;
        let reader = BufReader::new(file);
        Ok(VcfReader::new(reader))
    }

    pub fn new(inner: R) -> VcfReader<R> {
        VcfReader {
            inner,
            line_no: 0,
            meta: None,
            headers: None,
            has_format: false,
        }
    }

    pub fn read_meta(&mut self) -> io::Result<()> {
        let mut line = String::with_capacity(DEFAULT_BUF_SIZE);
        let mut meta = String::with_capacity(DEFAULT_BUF_SIZE);

        let headers: String;

        self.read_line(&mut line)?;

        if !line.starts_with(FILE_FORMAT) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid VCF file format",
            ));
        }

        meta.push_str(&line);

        loop {
            self.read_line(&mut line)?;

            if line.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    String::from("unexpected EOF"),
                ));
            }

            if line.starts_with(META_COMMENT) {
                meta.push_str(&line);
            } else if line.starts_with(&MANDATORY_HEADERS[0]) {
                headers = line.clone();
                self.has_format = has_format(&line);
                break;
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    String::from("unexpected non-header line"),
                ));
            }
        }

        self.meta = Some(meta);
        self.headers = Some(headers);

        Ok(())
    }

    pub fn meta(&self) -> Option<&str> {
        match self.meta {
            Some(ref m) => Some(m),
            None => None,
        }
    }

    pub fn headers(&self) -> Option<&str> {
        match self.headers {
            Some(ref h) => Some(h),
            None => None,
        }
    }

    pub fn mandatory_headers(&self) -> Option<Vec<&str>> {
        if self.headers.is_none() {
            return None;
        }

        let mut headers = MANDATORY_HEADERS.to_vec();

        if self.has_format {
            headers.push(OPTIONAL_HEADER);
        }

        Some(headers)
    }

    pub fn samples(&self) -> Option<Vec<&str>> {
        self.headers.as_ref().map(|h| {
            let n_headers = self.n_headers();
            let pieces = h.trim().split('\t');
            pieces.skip(n_headers).collect()
        })
    }

    pub fn inner(&self) -> &R {
        &self.inner
    }

    pub fn into_inner(self) -> R {
        self.inner
    }

    pub fn n_headers(&self) -> usize {
        let len = MANDATORY_HEADERS.len();

        if self.has_format {
            len + 1
        } else {
            len
        }
    }

    fn read_line(&mut self, buf: &mut String) -> io::Result<usize> {
        self.line_no += 1;
        buf.clear();
        self.inner.read_line(buf)
    }
}

fn has_format(headers: &str) -> bool {
    headers.split('\t')
        .skip(MANDATORY_HEADERS.len())
        .next()
        .map(|header| header == OPTIONAL_HEADER)
        .unwrap_or(false)
}

#[test]
fn test_has_format() {
    let headers = "";
    assert!(!has_format(headers));

    let headers = "#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\n";
    assert!(!has_format(headers));

    let headers = "#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\tFORMAT\tSJACT001_D\n";
    assert!(has_format(headers));
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{self, BufReader};
    use std::path::Path;

    use super::VcfReader;

    fn read_vcf<P>(path: P) -> io::Result<VcfReader<BufReader<File>>> where P: AsRef<Path> {
        let mut reader = VcfReader::<BufReader<File>>::open(path).unwrap();
        reader.read_meta()?;
        Ok(reader)
    }

    #[test]
    fn test_read_meta_with_no_file_format() {
        assert!(read_vcf("test/fixtures/sample.empty.vcf").is_err());
    }

    #[test]
    fn test_read_meta_with_no_headers() {
        assert!(read_vcf("test/fixtures/sample.invalid.vcf").is_err());
    }

    #[test]
    fn test_mandatory_headers() {
        let reader = read_vcf("test/fixtures/sample.single.vcf").unwrap();
        let headers = reader.mandatory_headers().unwrap();
        assert_eq!(headers.len(), 9);
    }

    #[test]
    fn test_samples() {
        let reader = read_vcf("test/fixtures/sample.single.vcf").unwrap();
        let samples = reader.samples().unwrap();
        assert_eq!(samples.len(), 1);
        assert_eq!(samples[0], "SJACT001_D");

        let reader = read_vcf("test/fixtures/sample.multi.vcf").unwrap();
        let samples = reader.samples().unwrap();
        assert_eq!(samples, vec!["SJACT001_D", "SJACT002_D", "SJACT003_D"]);
    }
}
