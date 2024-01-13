mod genomic_processing;
use genomic_processing::FASTQParser;

fn main() {
    let fastq_path: &str = "C:/Users/ellio/OneDrive/Documents/GitHub/Genetic-Modeling-in-Rust/genomic_processing/src/example.fastq";
    let parser: Result<FASTQParser<'_>, genomic_processing::FASTQErr> = FASTQParser::new(fastq_path);
    match parser {
        Ok(parser) => {
            parser.parse();
        }
        Err(e) => {
            println!("Error creating FASTQParser: {}", e);
        }
    }
}
