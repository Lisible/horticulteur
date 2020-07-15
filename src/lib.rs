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

use std::iter::Peekable;
use std::str::Chars;

pub type CSV = Vec<CSVRecord>;
pub type CSVRecord = Vec<CSVField>;
pub type CSVField = String;

const COMMA: char = ',';
const DQUOTE: char = '"';
const CR: char = '\r';
const LF: char = '\n';

#[derive(Debug)]
pub enum Error {
    UnexpectedCharacterInEscapedField(char),
    UnexpectedCharacterInNonEscapedField(char),
    UnexpectedEndOfInput,
    AteWrongCharacter(char, char),
}

/// Parses a CSV
///
/// Returns [`Result<CSV,Error>::Ok`] if the CSV was parsed successfully,
/// [`Result<CSV, Error>::Err`] if a parse error occurred
pub fn parse_csv(input_csv: &str) -> Result<CSV, Error> {
    let csv_string = input_csv.trim();
    let mut csv_iterator = csv_string.chars().peekable();
    let mut csv = CSV::new();
    csv.push(parse_record(&mut csv_iterator)?);
    while let Some(_) = csv_iterator.peek() {
        eat(CR, &mut csv_iterator)?;
        eat(LF, &mut csv_iterator)?;
        if let Some(_) = csv_iterator.peek() {
            csv.push(parse_record(&mut csv_iterator)?);
        }
    }
    Ok(csv)
}

fn parse_record(csv_iterator: &mut Peekable<Chars>) -> Result<CSVRecord, Error> {
    let mut record = CSVRecord::new();
    record.push(parse_field(csv_iterator)?);
    while let Some(&COMMA) = csv_iterator.peek() {
        eat(COMMA, csv_iterator)?;
        record.push(parse_field(csv_iterator)?);
    }
    Ok(record)
}

fn parse_field(csv_iterator: &mut Peekable<Chars>) -> Result<CSVField, Error> {
    if let Some(&DQUOTE) = csv_iterator.peek() {
        parse_escaped_field(csv_iterator)
    } else {
        parse_non_escaped_field(csv_iterator)
    }
}

fn parse_escaped_field(csv_iterator: &mut Peekable<Chars>) -> Result<CSVField, Error> {
    let mut field = CSVField::new();
    eat(DQUOTE, csv_iterator)?;
    while let Some(character) = csv_iterator.next() {
        if is_textdata(character) || character == COMMA || character == CR || character == LF {
            field.push(character);
        } else if character == DQUOTE {
            if let Some(&DQUOTE) = csv_iterator.peek() {
                field.push(character);
                csv_iterator.next();
            } else {
                break;
            }
        }
    }

    Ok(field)
}

fn parse_non_escaped_field(csv_iterator: &mut Peekable<Chars>) -> Result<CSVField, Error> {
    let mut field = CSVField::new();
    while let Some(character) = csv_iterator.peek() {
        if *character == COMMA {
            break;
        } else if is_textdata(*character) {
            field.push(*character);
            csv_iterator.next();
        } else if *character == CR {
            break;
        } else {
            return Err(Error::UnexpectedCharacterInNonEscapedField(*character));
        }
    }

    Ok(field)
}

fn eat(character: char, character_iterator: &mut Peekable<Chars>) -> Result<(), Error> {
    if let Some(c) = character_iterator.next() {
        if c == character {
            Ok(())
        } else {
            Err(Error::AteWrongCharacter(character, c))
        }
    } else {
        Err(Error::UnexpectedEndOfInput)
    }
}

fn is_textdata(character: char) -> bool {
    character == '\u{0020}'
        || character == '\u{0021}'
        || (character >= '\u{0023}' && character <= '\u{002B}')
        || (character >= '\u{002D}' && character <= '\u{007E}')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        assert_eq!(parse_csv("").unwrap(), vec![vec!["".into()] as CSVRecord]);
    }

    #[test]
    fn parse_single_field_non_escaped() {
        assert_eq!(
            parse_csv("value").unwrap(),
            vec![vec!["value".into()] as CSVRecord]
        );
    }

    #[test]
    fn parse_multiple_fields_non_escaped() {
        assert_eq!(
            parse_csv("a,b,c").unwrap(),
            vec![vec!["a".into(), "b".into(), "c".into()] as CSVRecord]
        );
    }

    #[test]
    fn parse_multiple_fields_escaped() {
        assert_eq!(
            parse_csv("\"a\"\"\",\"\"\"b\"\"\",\"c  \r\ndef\"").unwrap(),
            vec![vec!["a\"".into(), "\"b\"".into(), "c  \r\ndef".into()] as CSVRecord]
        );
    }

    #[test]
    fn parse_multiple_records() {
        assert_eq!(
            parse_csv("5,10,15\r\n20,25,30").unwrap(),
            vec![
                vec!["5".into(), "10".into(), "15".into()] as CSVRecord,
                vec!["20".into(), "25".into(), "30".into()] as CSVRecord
            ]
        );
    }

    #[test]
    fn parse_multiple_records_non_escaped() {
        assert_eq!(
            parse_csv("\"5\",10,15\r\n20,\"25\"\"baguette\"\" \r \"\"\n \r\n\",30").unwrap(),
            vec![
                vec!["5".into(), "10".into(), "15".into()] as CSVRecord,
                vec![
                    "20".into(),
                    "25\"baguette\" \r \"\n \r\n".into(),
                    "30".into()
                ] as CSVRecord
            ]
        );
    }
}
