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
    pub raw_data: Vec<u8>
}

#[derive(Debug)]
pub struct FASTQObj {
    pub seq_id: String,
    pub nucleotide_sequence: Vec<u8>,
    pub quality_score: Vec<u8>
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
                    let mut contents: Vec<u8> = Vec::new();
                    file.read_to_end(&mut contents)
                        .map(|_| FASTQParser { fastq_path, raw_data: contents })
                        .map_err(|_| FASTQErr::ReadFail)
                })
        } else {
            Err(FASTQErr::NotFastq)
        }
    }

    /// FASTQ files store quality scores in Phred Codes. 
    /// 
    /// This method decodes them such that they can be processed more easily by the FASTQObj
    fn phred_decode(&self, scores: &[u8]) -> Vec<u8> {
        scores.iter().map(|&char_code| char_code - 33).collect()
    }

    /// Parses the raw data stored in the FASTQParser, then returns a vector containing (in this order)
    /// 
    /// the sequence id, the nucleotide sequence, and the quality score.
    /// 
    /// Each element of the return Vector has a type such that it is easily applicable to the FASTQObj object
    pub fn parse(&self) -> (String, Vec<u8>, Vec<u8>) {
        let lines: Vec<&[u8]> = self.raw_data.split(|&c| c == b'\n').collect();
        let sequence_id = String::from_utf8_lossy(lines[0]).into_owned();
        let nucleotide_sequence = Vec::from(lines[1]);
        let quality_score = self.phred_decode(lines[3]);
        println!("Sequence ID: {}", sequence_id);
        println!("Nucleotide Sequence: {:?}", nucleotide_sequence);
        println!("Quality Score: {:?}", quality_score.clone());
        (sequence_id, nucleotide_sequence, quality_score)
    }    
}