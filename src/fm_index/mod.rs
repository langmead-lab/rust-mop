const TERMINAL_CHAR: char = '$';

pub struct FMIndex {
    alphabet: Vec<char>,        // A vector of characters representing the alphabet of the input text.
    counts: Vec<usize>,         // A vector of integers representing the count of each character in the alphabet.
    suffix_array: Vec<usize>,   // A vector of integers representing the suffix array of the input text.
    bwt: String,                // A string representing the Burrows-Wheeler Transform of the input text.
    tallies: Vec<Vec<usize>>    // A vector of vectors of integers, where each inner vector corresponds to a character in the alphabet.
}

impl FMIndex {

    /// Initializes a new empty FM-Index.
    pub fn new() -> Self {
        FMIndex{
            alphabet: Vec::new(),
            counts: Vec::new(),
            suffix_array: Vec::new(),
            bwt: String::new(),
            tallies: Vec::new(),
        }
    }

    /// Stores the alphabet of the input text in self.alphabet.
    pub fn find_alphabet(&mut self, text: &str) -> &Vec<char> {
        unimplemented!("Implement find_alphabet")
    }

    /// Stores the count of each character from self.alphabet in self.counts.
    ///
    /// Requires that find_alphabet has been called first.
    pub fn compute_counts(&mut self, text: &str) -> &Vec<usize> {
        unimplemented!("Implement compute_counts")
    }

    /// Constructs the suffix array of the input text and stores it in self.suffix_array.
    pub fn construct_suffix_array(&mut self, text: &str) -> &Vec<usize> {
        unimplemented!("Implement suffix array construction")
    }

    /// Builds and returns the Burrows-Wheeler Matrix using self.suffix_array and the input text.
    ///
    /// Note: This method returns the matrix but does not store it in the FM-Index.
    /// Requires that construct_suffix_array has been called first.
    pub fn construct_bwm(&self, text: &str) -> Vec<String> {
        unimplemented!("Implement Burrows-Wheeler Matrix construction from suffix array")
    }

    /// Prints the Burrows-Wheeler Matrix using self.suffix_array and the input text.
    ///
    /// Requires that construct_suffix_array has been called first and construct_bwm has been implemented.
    pub fn print_bwm(&self, text: &str) {
        for (i, row) in self.construct_bwm(text).iter().enumerate() {
            println!("{}:\t{}", i, row);
        }
    }

    /// Constructs the Burrows-Wheeler Transform using self.suffix_array and the input text
    /// and stores it in self.bwt.
    ///
    /// Requires that construct_suffix_array has been called first.
    pub fn construct_bwt(&mut self, text: &str) -> &str {
        unimplemented!("Implement Burrows-Wheeler Transform construction from suffix array")
    }

    /// Computes and returns the number of runs in the BWT (r).
    ///
    /// Requires that construct_bwt has been called first.
    pub fn compute_r(&self) -> usize {
        unimplemented!("Implement compute_r")
    }

    /// Constructs the tallies vectors for each non-terminal character of the BWT and stores them in self.tallies.
    ///
    /// Requires that construct_bwt and find_alphabet have been called first.
    pub fn construct_tallies(&mut self) -> &Vec<Vec<usize>> {
        unimplemented!("Implement construct_tallies")
    }

    /// Builds the FM-Index from the input text.
    ///
    /// This method requires the following methods to be implemented:
    /// - find_alphabet
    /// - compute_counts
    /// - construct_suffix_array
    /// - construct_bwt
    /// - construct_tallies
    pub fn build(&mut self, text: &str) {
        self.find_alphabet(text);
        self.compute_counts(text);
        self.construct_suffix_array(text);
        self.construct_bwt(text);
        self.construct_tallies();
    }

    /// Returns the result of LF-mapping (Last-to-First column mapping) at the given bwt_offset.
    /// 
    /// Requires that build has been called first.
    pub fn lf_mapping(&self, bwt_offset: usize) -> usize {
        unimplemented!("Implement LF-mapping")
    }

    /// Reconstructs and returns the original text from the BWT.
    /// 
    /// Requires that build has been called first.
    pub fn inverse_bwt(&self) -> String {
        unimplemented!("Implement inverse BWT")
    }

    /// Searches the pattern in the text.
    /// Returns a BWM interval as a tuple (start, end) if the pattern is found, otherwise returns None.
    /// 
    /// Requires that build has been called first and lf_mapping has been implemented.
    pub fn backward_search(&self, pattern: &str) -> Option<(usize, usize)> {
        unimplemented!("Implement backward search using FM-index")
    }

    /// Returns the number of occurrences of a pattern in the text using backward_search.
    /// 
    /// Requires that build has been called first and backward_search has been implemented.
    pub fn count_occurrences(&self, pattern: &str) -> usize {
        unimplemented!("Implement pattern counting using backward search")
    }

}