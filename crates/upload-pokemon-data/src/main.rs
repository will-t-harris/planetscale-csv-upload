mod db;
mod pokemon_csv;
use db::*;
use pokemon_csv::*;

fn main() -> Result<(), csv::Error> {
    let mut reader = csv::Reader::from_path("./crates/upload-pokemon-data/pokemon.csv")?;

    for result in reader.deserialize() {
        let record: PokemonCsv = result?;
        let pokemon_row: PokemonTableRow = record.into();
        println!("{:?}", pokemon_row);
    }

    Ok(())
}
