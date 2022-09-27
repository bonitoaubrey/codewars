pub fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;

    for i in string.as_bytes() {
        if *i == b'a' || *i == b'e' || *i == b'o' || *i == b'u' || *i == b'i' {
            vowels_count += 1;
        }
    }

    vowels_count
}
