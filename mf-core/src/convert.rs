use std::fs::File;

use crate::job::ConvertJob;

pub fn convert(cj: ConvertJob) {
    let mut input_file = File::open(cj.input);
}
