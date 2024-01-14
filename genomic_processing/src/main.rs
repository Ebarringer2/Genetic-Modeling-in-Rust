mod genomic_processing;
use genomic_processing::FASTQParser;
use genomic_processing::FASTQObj;

fn main() {
    let fastq_path: &str = "C:/Users/ellio/OneDrive/Documents/GitHub/Genetic-Modeling-in-Rust/genomic_processing/src/example.fastq";
    let parser: Result<FASTQParser<'_>, genomic_processing::FASTQErr> = FASTQParser::new(fastq_path);
    match parser {
        Ok(parser) => {
            let attributes: (String, Vec<u8>, Vec<u8>) = parser.parse();
            let fastq_obj: FASTQObj = FASTQObj::new(attributes.0, attributes.1, attributes.2);
            let reversed_sequence: Vec<u8> = fastq_obj.reverse(fastq_obj.nucleotide_sequence.clone());
            let complemented_sequence: Vec<u8> = fastq_obj.complement(fastq_obj.nucleotide_sequence.clone());
            let reverse_complement_sequence: Vec<u8> = fastq_obj.reverse_complement(fastq_obj.nucleotide_sequence.clone());
            println!("\n\n\n");
            println!("Original Nucleotide Sequence: {:#?}", fastq_obj.nucleotide_sequence);
            println!("Reversed Nucleotide Sequence: {:#?}", reversed_sequence);
            println!("Complemented Nucleotide Sequence: {:#?}", complemented_sequence);
            println!("Reverse Complemented Nucleotide Sequence: {:#?}", reverse_complement_sequence);
        }
        Err(e) => {
            println!("Error creating FASTQParser: {}", e);
        }
    }
}
