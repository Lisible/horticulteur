use horticulteur::{parse_csv, Error};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let csv: &'static str = "\"1\",\"2\",\"3\"\r\n\"4\",\"5\",\"6\"";
    let parsed_csv = parse_csv(csv)?;
    let sums: Vec<i32> = parsed_csv
        .iter()
        .map(|record| {
            record
                .iter()
                .fold(0, |acc, v| acc + i32::from_str(v).unwrap())
        })
        .collect();

    for (i, sum) in sums.iter().enumerate() {
        println!("Sum {}: {}", i, sum);
    }

    assert_eq!(sums.len(), 2);
    assert_eq!(sums.get(0), Some(&6));
    assert_eq!(sums.get(1), Some(&15));
    Ok(())
}
