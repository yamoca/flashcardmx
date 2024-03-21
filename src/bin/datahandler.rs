use csv;

mod dictionary_csv;
use dictionary_csv::*;


fn main() -> Result<(), csv::Error> {
    let mut wordlist: Vec<WordCsv> = vec![];

    let mut rdr = csv::Reader::from_path(
        "./src/bin/sampledata.csv"
    )?;
    for result in rdr.deserialize() {
        let record: WordCsv = result?;
        wordlist.push(record)
    }
    println!("{:?}", wordlist);
    Ok(())
}