use brix_processor::ProcessorCore;
use std::fs::read_to_string;
use std::path::PathBuf;

pub fn setup<'a>() -> ProcessorCore<'a> {
    ProcessorCore::new()
}

pub fn load_file(hbs_name: &str) -> Result<String, std::io::Error> {
    let path = PathBuf::from(format!("tests/templates/{}.hbs", hbs_name));
    read_to_string(path)
}

pub fn line_assert(contents: String, assertion: Vec<&str>) -> bool {
    let mut itr = 0;
    for line in contents.lines() {
        if assertion[itr] != line {
            return false;
        }
        itr += 1;
    }
    return true;
}
