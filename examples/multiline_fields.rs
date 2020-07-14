use horticulteur::{parse_csv, Error};

fn main() -> Result<(), Error> {
    let csv = std::fs::read_to_string("./examples/cogito.csv").unwrap();
    let parsed_csv = parse_csv(&csv)?;
    let record = parsed_csv.get(0).unwrap();
    assert_eq!(record.len(), 3);
    assert_eq!(record.get(0).unwrap(), "Je pense,\ndonc\nje suis.");
    assert_eq!(record.get(1).unwrap(), "I think,\ntherefore\nI am.");
    assert_eq!(record.get(2).unwrap(), "Cogito,\nergo\nsum.");

    Ok(())
}
