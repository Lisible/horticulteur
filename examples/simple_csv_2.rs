/*
 * MIT License
 *
 * Copyright (c) 2020 Clément S.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use horticulteur::*;

fn main() -> Result<(), Error> {
    let csv_string: &'static str = "1,2,3\r\n4,5,6";
    let parsed_csv: CSV = parse_csv(csv_string)?; // CSV is an alias for Vec<CSVRecord>
    let first_record: &CSVRecord = parsed_csv.get(0).unwrap(); // CSVRecord is an alias for Vec<CSVField>
    let first_field: &CSVField = first_record.get(0).unwrap(); // CSVField is an alias for String
    assert_eq!(first_field, "1");
    Ok(())
}
