mod get_count;
use crate::get_count::get_count;

#[test]
fn my_tests() {
    assert_eq!(get_count("abracadabra"), 5);
}
