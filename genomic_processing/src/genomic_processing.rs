use std::fs::File;
use std::io::Read;
use std::fmt;

#[derive(Debug)]
pub enum FASTQErr {
    NotFastq,
    ReadFail,
    OpenFail
}

impl fmt::Display for FASTQErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FASTQErr::NotFastq => write!(f, "File does not have extension .fastq"),
            FASTQErr::OpenFail => write!(f, "Unable to open .fastq file"),
            FASTQErr::ReadFail => write!(f, "Unable to read .fastq file")
        }
    }
}

#[derive(Debug)]
pub struct FASTQParser<'a> {
    pub fastq_path: &'a str,
    pub raw_data: String
}

#[derive(Debug)]
pub struct FASTQObj<'a> {
    pub seq_id: &'a str,
    pub nucleotide_sequence: &'a [u8],
    pub quality_score: &'a [u8]
}

impl<'a> FASTQParser<'a> {
    /// Creates a new FASTQParser object, which is used to read data from a .fastq file, validate it,
    /// 
    /// and then return data in specific formats that can be applied to the FASTQObj struct.
    pub fn new(fastq_path: &str) -> Result<FASTQParser, FASTQErr> {
        if fastq_path.ends_with(".fastq") {
            File::open(fastq_path)
                .map_err(|_| FASTQErr::OpenFail)
                .and_then(|mut file| {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)
                        .map(|_| FASTQParser { fastq_path, raw_data: contents })
                        .map_err(|_| FASTQErr::ReadFail)
                })
        } else {
            Err(FASTQErr::NotFastq)
        }
    }

    /// Parses the raw data stored in the FASTQParser, then returns a vector containing (in this order)
    /// 
    /// the sequence id, the nucleotide sequence, and the quality score.
    /// 
    /// Each element of the return Vector has a type such that it is easily applicable to the FASTQObj object
    pub fn parse(&self) -> (&str, &[u8], &[u8]) {
        let lines: Vec<&str> = self.raw_data.split('\n').collect();
        let sequence_id: &str = lines[0];
        let nucleotide_sequence: &[u8] = lines[1].as_bytes();
        let quality_score: &[u8] = lines[2].as_bytes();
        println!("Sequence ID: {}", sequence_id);
        println!("Nucleotide Sequence: {:#?}", nucleotide_sequence);
        println!("Quality Score: {:#?}", quality_score);
        (sequence_id, nucleotide_sequence, quality_score)
    }
}