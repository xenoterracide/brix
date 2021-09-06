use lazy_static::lazy_static;
use std::collections::HashMap;

mod common;

lazy_static! {
    static ref TO_LOWER_CONTEXT: HashMap<String, String> = {
        let mut map = HashMap::new();
        map.insert(String::from("one"), String::from("ALLUPPER"));
        map.insert(String::from("two"), String::from("already_lower"));
        map.insert(String::from("three"), String::from("mIxEd.cAsE"));
        map
    };
    static ref TO_LOWER_ASSERTIONS: Vec<&'static str> =
        vec!["allupper", "already_lower", "mixed.case"];
}

#[test]
fn to_lower() {
    let core = common::setup();
    let context = brix_processor::create_context(TO_LOWER_CONTEXT.clone());
    let contents = common::load_file("lower").unwrap();

    let result = core.process(contents, context).unwrap();
    assert!(common::line_assert(result, TO_LOWER_ASSERTIONS.to_vec()))
}
