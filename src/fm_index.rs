const SENTINEL: char = '$';

#[derive(Debug, PartialEq)]
pub struct FMIndex {
    alphabet: Vec<char>,
    counts: Vec<usize>,
    suffix_array: Vec<usize>,
    bwt: String,
    tallies: Vec<Vec<usize>>
}

impl FMIndex {

    /// Creates a new BWTIndex from the input text
    pub fn new() -> Self {
        FMIndex{
            alphabet: Vec::new(),
            counts: Vec::new(),
            suffix_array: Vec::new(),
            bwt: String::new(),
            tallies: Vec::new(),
        }
    }

    /// Builds the FMIndex using the methods that must be implemented
    pub fn build(&mut self, text: &str) {
        self.find_alphabet(text);
        self.compute_counts(text);
        self.construct_suffix_array(text);
        self.construct_bwt(text);
        self.construct_tallies();
    }

    pub fn print_bwm(&self, text: &str) {
        for (i, row) in self.construct_bwm(text).iter().enumerate() {
            println!("{}:\t{}", i, row);
        }
    }

        /// Finds the alphabet of the input text
    pub fn find_alphabet(&mut self, text: &str) -> &Vec<char> {
        unimplemented!("Implement find_alphabet")
    }

    /// Computes the counts of each character in the input text
    pub fn compute_counts(&mut self, text: &str) -> &Vec<usize> {
        unimplemented!("Implement compute_counts")
    }

    /// Constructs the suffix array for the input text
    pub fn construct_suffix_array(&mut self, text: &str) -> &Vec<usize> {
        unimplemented!("Implement suffix array construction")
    }

    /// Constructs the Burrows-Wheeler Matrix of the input text from the suffix array
    pub fn construct_bwm(&self, text: &str) -> Vec<String> {
        unimplemented!("Implement Burrows-Wheeler Matrix construction from suffix array")
    }

    /// Constructs the Burrows-Wheeler Transform of the input text from the suffix array
    pub fn construct_bwt(&mut self, text: &str) -> &str {
        unimplemented!("Implement Burrows-Wheeler Transform construction from suffix array")
    }

    /// Compute number of runs in the BWT
    pub fn compute_r(&self) -> usize {
        unimplemented!("Implement compute_r")
    }

    /// Constructs the tallies vectors for each character of the BWT
    pub fn construct_tallies(&mut self) -> &Vec<Vec<usize>> {
        unimplemented!("Implement construct_tallies")
    }

    /// Performs LF-mapping (Last-to-First column mapping)
    pub fn lf_mapping(&self, last_row: usize) -> usize {
        unimplemented!("Implement LF-mapping")
    }

    /// Reconstructs the original text from BWT
    pub fn inverse_bwt(&self) -> String {
        unimplemented!("Implement inverse BWT")
    }

    /// Performs backward search
    pub fn backward_search(&self, pattern: &str) -> Option<(usize, usize)> {
        unimplemented!("Implement backward search using FM-index")
    }

    /// Counts occurrences of a pattern in the text using backward search
    pub fn count_occurrences(&self, pattern: &str) -> usize {
        unimplemented!("Implement pattern counting using backward search")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_alphabet() {
        let text = "banana";
        let mut index = FMIndex::new();

        assert_eq!(*index.find_alphabet(text), vec!['a', 'b', 'n']);
    }

    #[test]
    fn test_compute_counts() {
        let text = "banana";
        let mut index = FMIndex::new();
        index.find_alphabet(text);

        assert_eq!(*index.compute_counts(text), vec![3, 1, 2]);
    }

    #[test]
    fn test_construct_suffix_array() {
        let text = "banana";
        let mut index = FMIndex::new();

        assert_eq!(*index.construct_suffix_array(text), vec![6, 5, 3, 1, 0, 4, 2]);
    }

    #[test]
    fn test_construct_bwm() {
        let text = "banana";
        let mut index = FMIndex::new();
        index.construct_suffix_array(text);
        // index.print_bwm(text);

        assert_eq!(index.construct_bwm(text), vec![
                                                    "$banana",
                                                    "a$banan",
                                                    "ana$ban",
                                                    "anana$b",
                                                    "banana$",
                                                    "na$bana",
                                                    "nana$ba"]);
    }

    #[test]
    fn test_construct_bwt() {
        let text = "banana";
        let mut index = FMIndex::new();
        index.construct_suffix_array(text);

        assert_eq!(index.construct_bwt(text), "annb$aa");
    }

    #[test]
    fn test_compute_r() {
        let text = "banana";
        let mut index = FMIndex::new();
        index.construct_suffix_array(text);
        index.construct_bwt(text);

        assert_eq!(index.compute_r(), 5);
    }

    #[test]
    fn test_construct_tallies() {
        let text = "banana";
        let mut index = FMIndex::new();
        index.construct_suffix_array(text);
        index.construct_bwt(text);
        index.find_alphabet(text);
        index.compute_counts(text);

        assert_eq!(*index.construct_tallies(), vec![
                        vec![1, 1, 1, 1, 1, 2, 3],
                        vec![0, 0, 0, 1, 1, 1, 1],
                        vec![0, 1, 2, 2, 2, 2, 2]
        ]);
    }

    #[test]
    fn test_lf_mapping() {
        let text = "banana";
        let mut index = FMIndex::new();
        index.build(text);

        assert_eq!(index.lf_mapping(5), 2);
        assert_eq!(index.lf_mapping(0), 1);
        assert_eq!(index.lf_mapping(4), 0);
        assert_eq!(index.lf_mapping(3), 4);
        assert_eq!(index.lf_mapping(6), 3);
        assert_eq!(index.lf_mapping(1), 5);
        assert_eq!(index.lf_mapping(2), 6);
    }

    #[test]
    fn test_inverse_bwt() {
        let text = "banana";
        let mut index = FMIndex::new();
        index.build(text);
        // index.print_bwm(text);

        assert_eq!(index.inverse_bwt(), text);
    }

    #[test]
    fn test_backward_search() {
        let text = "mississippi";
        let mut index = FMIndex::new();
        index.build(text);
        // index.print_bwm(text);

        assert_eq!(index.backward_search("ssi"), Some((10, 11)));
        assert_eq!(index.backward_search("sssi"), None);
        assert_eq!(index.backward_search("$"), Some((0, 0)));
        assert_eq!(index.backward_search("mi"), Some((5, 5)));
    }

    #[test]
    fn test_count_occurrences() {
        let text = "mississippi";
        let mut index = FMIndex::new();
        index.build(text);
        // index.print_bwm(text);

        assert_eq!(index.count_occurrences("si"), 2);
        assert_eq!(index.count_occurrences("mis"), 1);
        assert_eq!(index.count_occurrences("xyz"), 0);
        assert_eq!(index.count_occurrences("p"), 2);
        assert_eq!(index.count_occurrences("s"), 4);
        assert_eq!(index.count_occurrences("issi"), 2);
        assert_eq!(index.count_occurrences("issi$"), 0);
    }
}