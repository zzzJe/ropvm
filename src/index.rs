use indexmap::IndexSet;
use regex::Regex;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum ParsedRange<'a> {
    Single(usize),
    Inclusive(usize, usize),
    From(usize),
    To(usize),
    Full,
    Invalid(&'a str, String),
}

pub fn parse(raw_index: &str, boundary: usize) -> (Vec<ParsedRange>, Vec<(String, String)>) {
    let mut valid_index = Vec::new();
    let mut invalid_index = Vec::new();

    // if the index is "", namely, user enter `a.b.c[]`, see is as `a.b.c[1]`
    if raw_index.is_empty() {
        valid_index.push(ParsedRange::Single(1));
        return (valid_index, invalid_index);
    }

    // for single value
    let num_re = Regex::new(r"^\d$").unwrap();
    // for range value
    let range_re = Regex::new(r"^(\d*)~(\d*)$").unwrap();
    let indices: Vec<&str> = raw_index.split('/').filter(|e| !e.is_empty()).collect();

    for index in indices {
        let result = if num_re.is_match(index) {
            let num: usize = index.parse().unwrap();
            if num > boundary || num == 0 {
                ParsedRange::Invalid(index, format!("Range should between 1~{boundary}"))
            } else {
                ParsedRange::Single(num)
            }
        } else if let Some(captures) = range_re.captures(index) {
            let first = captures.get(1).unwrap().as_str().parse::<usize>();
            let second = captures.get(2).unwrap().as_str().parse::<usize>();
            match (first, second) {
                (Ok(n), Err(_)) | (Err(_), Ok(n)) if n > boundary || n == 0 => {
                    ParsedRange::Invalid(index, format!("Range should between 1~{boundary}"))
                }
                (Ok(n1), Ok(n2)) if n1 > boundary || n2 > boundary || n1 == 0 || n2 == 0 => {
                    ParsedRange::Invalid(index, format!("Range should between 1~{boundary}"))
                }
                (Ok(n1), Ok(n2)) if n1 > n2 => ParsedRange::Invalid(
                    index,
                    format!(
                        "The starting index({n1}) must not greater than the ending index({n2})"
                    ),
                ),
                (Err(_), Err(_)) => ParsedRange::Full,
                (Ok(n), Err(_)) => ParsedRange::From(n),
                (Err(_), Ok(n)) => ParsedRange::To(n),
                (Ok(n1), Ok(n2)) => ParsedRange::Inclusive(n1, n2),
            }
        } else {
            ParsedRange::Invalid(index, format!("Syntax error"))
        };
        if let ParsedRange::Invalid(raw, reason) = result {
            invalid_index.push((raw.to_string(), reason));
        } else {
            valid_index.push(result);
        }
    }
    (valid_index, invalid_index)
}
pub fn purify(valid_ranges: Vec<ParsedRange>, boundary: usize) -> IndexSet<usize> {
    let mut result = IndexSet::new();
    for range in valid_ranges {
        if let ParsedRange::Single(n) = range {
            result.insert(n);
        } else if let ParsedRange::Full = range {
            result.extend(1..=boundary);
        } else if let ParsedRange::To(n) = range {
            result.extend(1..=n);
        } else if let ParsedRange::From(n) = range {
            result.extend(n..=boundary);
        } else if let ParsedRange::Inclusive(n1, n2) = range {
            result.extend(n1..=n2);
        }
    }
    result
}
pub fn destruct_input(with_index: &str) -> Option<(String, String)> {
    let re = Regex::new(r"(.+)\[(.*)\]").unwrap();
    if re.is_match(with_index) {
        let capture = re.captures(with_index).unwrap();
        let mc_ver = capture.get(1).unwrap().as_str().to_string();
        let index = capture.get(2).unwrap().as_str().to_string();
        Some((mc_ver, index))
    } else {
        None
    }
}
