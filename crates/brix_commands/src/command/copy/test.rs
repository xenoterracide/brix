use spectral::assert_that;
use spectral::boolean::BooleanAssertions;

#[test]
fn my_test() {
    assert_that!(true).is_false();
}
