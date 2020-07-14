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
