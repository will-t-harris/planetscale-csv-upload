mod pokemon_csv;
use pokemon_csv::*;

fn main() -> Result<(), csv::Error> {
    let mut reader = csv::Reader::from_path("./crates/upload-pokemon-data/pokemon.csv")?;

    for result in reader.deserialize() {
        let record: PokemonCsv = result?;
        println!("{:?}", record);
    }

    Ok(())
}
