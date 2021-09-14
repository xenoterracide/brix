// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use lazy_static::lazy_static;
use std::collections::HashMap;

mod common;

lazy_static! {
    static ref TO_TILE_CONTEXT: HashMap<String, String> = {
        let mut map = HashMap::new();
        map.insert(String::from("one"), String::from("oneword"));
        map.insert(
            String::from("two"),
            String::from("Already in the Title Case"),
        );
        map.insert(String::from("three"), String::from("I AM YELLING"));
        map
    };
    static ref TO_TITLE_ASSERTIONS: Vec<&'static str> =
        vec!["Oneword", "Already in the Title Case", "I Am Yelling"];
}

#[test]
fn to_title() {
    let core = common::setup();
    let context = brix_processor::create_context(TO_TILE_CONTEXT.clone());
    let contents = common::load_file("title").unwrap();

    let result = core.process(contents, context).unwrap();
    assert!(common::line_assert(result, TO_TITLE_ASSERTIONS.to_vec()))
}
