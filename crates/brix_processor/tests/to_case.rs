// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use lazy_static::lazy_static;
use std::collections::HashMap;

mod common;

lazy_static! {
    static ref TO_CASE_CONTEXT: HashMap<String, String> = {
        let mut map = HashMap::new();
        map.insert(String::from("one"), String::from("tHIS iS tOGGLE cASE"));
        map.insert(String::from("two"), String::from("ThisIsPascalCase"));
        map.insert(String::from("three"), String::from("thisIsCamelCase"));
        map.insert(String::from("four"), String::from("ThisIsUpperCamelCase"));
        map.insert(String::from("five"), String::from("this_is_snake_case"));
        map.insert(
            String::from("six"),
            String::from("THIS_IS_UPPER_SNAKE_CASE"),
        );
        map.insert(
            String::from("seven"),
            String::from("THIS_IS_SCREAMING_SNAKE_CASE"),
        );
        map.insert(String::from("eight"), String::from("this-is-kebab-case"));
        map.insert(String::from("nine"), String::from("THIS-IS-COBOL-CASE"));
        map.insert(String::from("ten"), String::from("This-Is-Train-Case"));
        map.insert(String::from("eleven"), String::from("thisisflatcase"));
        map.insert(String::from("twelve"), String::from("THISISUPPERFLATCASE"));
        map.insert(
            String::from("thirteen"),
            String::from("tHiS iS aLtErNaTiNg CaSe"),
        );
        map
    };
    static ref TO_CASE_ASSERTIONS: Vec<&'static str> = vec![
        "t hIS i s t oGGLE c aSE",
        "tHIS iS pASCAL cASE",
        "tHIS iS cAMEL cASE",
        "tHIS iS uPPER cAMEL cASE",
        "tHIS iS sNAKE cASE",
        "tHIS iS uPPER sNAKE cASE",
        "tHIS iS sCREAMING sNAKE cASE",
        "tHIS iS kEBAB cASE",
        "tHIS iS cOBOL cASE",
        "tHIS iS tRAIN cASE",
        "tHISISFLATCASE",
        "tHISISUPPERFLATCASE",
        "t hI s i s a lT eR nA tI nG cA sE",
        "THisISTOggleCAse",
        "ThisIsPascalCase",
        "ThisIsCamelCase",
        "ThisIsUpperCamelCase",
        "ThisIsSnakeCase",
        "ThisIsUpperSnakeCase",
        "ThisIsScreamingSnakeCase",
        "ThisIsKebabCase",
        "ThisIsCobolCase",
        "ThisIsTrainCase",
        "Thisisflatcase",
        "Thisisupperflatcase",
        "THiSISALtErNaTiNgCaSe",
        "tHisISTOggleCAse",
        "thisIsPascalCase",
        "thisIsCamelCase",
        "thisIsUpperCamelCase",
        "thisIsSnakeCase",
        "thisIsUpperSnakeCase",
        "thisIsScreamingSnakeCase",
        "thisIsKebabCase",
        "thisIsCobolCase",
        "thisIsTrainCase",
        "thisisflatcase",
        "thisisupperflatcase",
        "tHiSISALtErNaTiNgCaSe",
        "THisISTOggleCAse",
        "ThisIsPascalCase",
        "ThisIsCamelCase",
        "ThisIsUpperCamelCase",
        "ThisIsSnakeCase",
        "ThisIsUpperSnakeCase",
        "ThisIsScreamingSnakeCase",
        "ThisIsKebabCase",
        "ThisIsCobolCase",
        "ThisIsTrainCase",
        "Thisisflatcase",
        "Thisisupperflatcase",
        "THiSISALtErNaTiNgCaSe",
        "t_his_i_s_t_oggle_c_ase",
        "this_is_pascal_case",
        "this_is_camel_case",
        "this_is_upper_camel_case",
        "this_is_snake_case",
        "this_is_upper_snake_case",
        "this_is_screaming_snake_case",
        "this_is_kebab_case",
        "this_is_cobol_case",
        "this_is_train_case",
        "thisisflatcase",
        "thisisupperflatcase",
        "t_hi_s_i_s_a_lt_er_na_ti_ng_ca_se",
        "T_HIS_I_S_T_OGGLE_C_ASE",
        "THIS_IS_PASCAL_CASE",
        "THIS_IS_CAMEL_CASE",
        "THIS_IS_UPPER_CAMEL_CASE",
        "THIS_IS_SNAKE_CASE",
        "THIS_IS_UPPER_SNAKE_CASE",
        "THIS_IS_SCREAMING_SNAKE_CASE",
        "THIS_IS_KEBAB_CASE",
        "THIS_IS_COBOL_CASE",
        "THIS_IS_TRAIN_CASE",
        "THISISFLATCASE",
        "THISISUPPERFLATCASE",
        "T_HI_S_I_S_A_LT_ER_NA_TI_NG_CA_SE",
        "T_HIS_I_S_T_OGGLE_C_ASE",
        "THIS_IS_PASCAL_CASE",
        "THIS_IS_CAMEL_CASE",
        "THIS_IS_UPPER_CAMEL_CASE",
        "THIS_IS_SNAKE_CASE",
        "THIS_IS_UPPER_SNAKE_CASE",
        "THIS_IS_SCREAMING_SNAKE_CASE",
        "THIS_IS_KEBAB_CASE",
        "THIS_IS_COBOL_CASE",
        "THIS_IS_TRAIN_CASE",
        "THISISFLATCASE",
        "THISISUPPERFLATCASE",
        "T_HI_S_I_S_A_LT_ER_NA_TI_NG_CA_SE",
        "t-his-i-s-t-oggle-c-ase",
        "this-is-pascal-case",
        "this-is-camel-case",
        "this-is-upper-camel-case",
        "this-is-snake-case",
        "this-is-upper-snake-case",
        "this-is-screaming-snake-case",
        "this-is-kebab-case",
        "this-is-cobol-case",
        "this-is-train-case",
        "thisisflatcase",
        "thisisupperflatcase",
        "t-hi-s-i-s-a-lt-er-na-ti-ng-ca-se",
        "T-HIS-I-S-T-OGGLE-C-ASE",
        "THIS-IS-PASCAL-CASE",
        "THIS-IS-CAMEL-CASE",
        "THIS-IS-UPPER-CAMEL-CASE",
        "THIS-IS-SNAKE-CASE",
        "THIS-IS-UPPER-SNAKE-CASE",
        "THIS-IS-SCREAMING-SNAKE-CASE",
        "THIS-IS-KEBAB-CASE",
        "THIS-IS-COBOL-CASE",
        "THIS-IS-TRAIN-CASE",
        "THISISFLATCASE",
        "THISISUPPERFLATCASE",
        "T-HI-S-I-S-A-LT-ER-NA-TI-NG-CA-SE",
        "T-His-I-S-T-Oggle-C-Ase",
        "This-Is-Pascal-Case",
        "This-Is-Camel-Case",
        "This-Is-Upper-Camel-Case",
        "This-Is-Snake-Case",
        "This-Is-Upper-Snake-Case",
        "This-Is-Screaming-Snake-Case",
        "This-Is-Kebab-Case",
        "This-Is-Cobol-Case",
        "This-Is-Train-Case",
        "Thisisflatcase",
        "Thisisupperflatcase",
        "T-Hi-S-I-S-A-Lt-Er-Na-Ti-Ng-Ca-Se",
        "thisistogglecase",
        "thisispascalcase",
        "thisiscamelcase",
        "thisisuppercamelcase",
        "thisissnakecase",
        "thisisuppersnakecase",
        "thisisscreamingsnakecase",
        "thisiskebabcase",
        "thisiscobolcase",
        "thisistraincase",
        "thisisflatcase",
        "thisisupperflatcase",
        "thisisalternatingcase",
        "THISISTOGGLECASE",
        "THISISPASCALCASE",
        "THISISCAMELCASE",
        "THISISUPPERCAMELCASE",
        "THISISSNAKECASE",
        "THISISUPPERSNAKECASE",
        "THISISSCREAMINGSNAKECASE",
        "THISISKEBABCASE",
        "THISISCOBOLCASE",
        "THISISTRAINCASE",
        "THISISFLATCASE",
        "THISISUPPERFLATCASE",
        "THISISALTERNATINGCASE",
        "t HiS i S t OgGlE c AsE",
        "tHiS iS pAsCaL cAsE",
        "tHiS iS cAmEl CaSe",
        "tHiS iS uPpEr CaMeL cAsE",
        "tHiS iS sNaKe CaSe",
        "tHiS iS uPpEr SnAkE cAsE",
        "tHiS iS sCrEaMiNg SnAkE cAsE",
        "tHiS iS kEbAb CaSe",
        "tHiS iS cObOl CaSe",
        "tHiS iS tRaIn CaSe",
        "tHiSiSfLaTcAsE",
        "tHiSiSuPpErFlAtCaSe",
        "t Hi S i S a Lt Er Na Ti Ng Ca Se",
    ];
}

#[test]
fn to_case() {
    let core = common::setup();
    let context = brix_processor::create_context(TO_CASE_CONTEXT.clone());
    let contents = common::load_file("case").unwrap();

    let result = core.process(contents, context).unwrap();
    assert!(common::line_assert(result, TO_CASE_ASSERTIONS.to_vec()))
}
