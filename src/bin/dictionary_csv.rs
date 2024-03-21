use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WordCsv {
    pub first_pp: String,
    pub second_pp: String,
    pub third_pp: String,
    pub fourfth_pp: String,
    pub infl_type: u8,
    pub infl_variant: u8,
}

fn main() {

}
