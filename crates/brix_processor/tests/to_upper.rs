// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use lazy_static::lazy_static;
use std::collections::HashMap;

mod common;

lazy_static! {
    static ref TO_UPPER_CONTEXT: HashMap<String, String> = {
        let mut map = HashMap::new();
        map.insert(String::from("one"), String::from("alllower"));
        map.insert(String::from("two"), String::from("ALREADY_UPPER"));
        map.insert(String::from("three"), String::from("mIxEd.cAsE"));
        map
    };
    static ref TO_UPPER_ASSERTIONS: Vec<&'static str> =
        vec!["ALLLOWER", "ALREADY_UPPER", "MIXED.CASE"];
}

#[test]
fn to_upper() {
    let core = common::setup();
    let context = brix_processor::create_context(TO_UPPER_CONTEXT.clone());
    let contents = common::load_file("upper").unwrap();

    let result = core.process(contents, context).unwrap();
    assert!(common::line_assert(result, TO_UPPER_ASSERTIONS.to_vec()))
}
