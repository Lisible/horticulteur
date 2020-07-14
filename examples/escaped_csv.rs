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
