use rust_mop::fm_index::FMIndex;

#[cfg(test)]
mod tests {
    use super::FMIndex;

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
    fn test_construct_bwt_emojis() {
        let text = "🍌🍍🍌🍌🍍🍌";
        let mut index = FMIndex::new();
        index.construct_suffix_array(text);

        assert_eq!(index.construct_bwt(text), "🍌🍍🍍🍌$🍌🍌");
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