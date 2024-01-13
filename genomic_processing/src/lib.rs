mod fastq_reading {

    use std::fs::File;
    use std::io::{self, Read};

    #[derive(Debug)]
    enum FASTQFormatErr {
        NotFastq
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
        /// and then return data in specific formats such that it can be applied to the FASTQObj struct
        pub fn new(fastq_path: &str) -> Result<FASTQParser, FASTQFormatErr> {
            match fastq_path {
                Some("fastq") => {

                }
                Some(_) => FASTQFormatErr::NotFastq
            }

        }

    }
}