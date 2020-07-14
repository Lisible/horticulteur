use horticulteur::*;

fn main() -> Result<(), Error> {
    let csv_string: &'static str = "1,2,3\r\n4,5,6";
    let parsed_csv: CSV = parse_csv(csv_string)?; // CSV is an alias for Vec<CSVRecord>
    let first_record: &CSVRecord = parsed_csv.get(0).unwrap(); // CSVRecord is an alias for Vec<CSVField>
    let first_field: &CSVField = first_record.get(0).unwrap(); // CSVField is an alias for String
    assert_eq!(first_field, "1");
    Ok(())
}
