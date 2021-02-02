use std::env;
use std::boxed::Box;

#[derive(Debug)]
struct ParsedStatement<'a> {
    table_name: String,
    value_pairs: Vec<(&'a str, &'a str)>
}

fn main() -> Result<(), String> {
    let insert_string = env::args().nth(1).unwrap();
    let parsed = parse_insert(&insert_string)?;

    let mut pad_length = 0;
    for pair in &parsed.value_pairs {
        pad_length = usize::max(pad_length, pair.0.len());
    }


    println!("INSERT INTO {}:", parsed.table_name);
    for pair in parsed.value_pairs {
        println!("  {:<pad$} : {}", pair.0, pair.1, pad = pad_length);
    }
    Ok(())
}

fn parse_insert<'a>(insert_string: &'a str) -> Result<ParsedStatement<'a>, String> {
    // Statement, splitted by whitespaces
    let mut splitted_iter = insert_string.split_ascii_whitespace();

    required_keyword(&splitted_iter.next(), "insert")?;
    required_keyword(&splitted_iter.next(), "into")?;

    let mut table_name = String::from(required_to_be_some(&splitted_iter.next(), "table name")?);

    let possible_alias = required_to_be_some(&splitted_iter.next(), "table alias or column names")?;

    // As far as I understand - we can only put it to Box, or use Either or smth.
    let first_column_names_iter: Box<dyn Iterator<Item=&'a str>>;

    if possible_alias.starts_with("(") {
        // If not alias
        // Then we already know that current statement starts with "(", and it is correct start for column names.
        first_column_names_iter = extract_open_bracket(&Some(possible_alias))?;
    } else {
        // If alias
        // Then next element should be tested to have "(".
        table_name.push_str(&format!(" {}", possible_alias));

        let column_names_first_part = required_to_be_some(&splitted_iter.next(), "column names")?;
        first_column_names_iter = extract_open_bracket(&Some(column_names_first_part))?;
    }

    // Current iterator with removed ( symbol
    let mut splitted_iter = Box::leak(first_column_names_iter).chain(splitted_iter);

    let column_names = parse_column_names(&mut splitted_iter)?;

    required_keyword(&splitted_iter.next(), "values")?;

    let first_values_iter = extract_open_bracket(&splitted_iter.next())?;

    let mut splitted_iter = Box::leak(first_values_iter).chain(splitted_iter);

    let values = parse_values(&mut splitted_iter)?;

    if column_names.len() != values.len() {
        return Err(format!("Expected {} values, got {}", column_names.len(), values.len()));
    }

    Ok(ParsedStatement{
        table_name: String::from(table_name),
        value_pairs: column_names.into_iter().zip(values.into_iter()).collect()
    })
}

fn required_keyword<'a>(statement: &Option<&'a str>, keyword: &'a str) -> Result<(), String> {
    match statement {
        None => {
            return Err(format!("Expected \"{}\" statement, instead statement ended", keyword));
        },
        Some(st) => {
            if st.to_lowercase().eq(&keyword.to_lowercase()) {
                return Ok(());
            } else {
                return Err(format!("Expected \"{}\" statement, instead got \"{}\"", keyword, keyword));
            }
        }
    }
}

fn required_to_be_some<'a>(statement: &Option<&'a str>, expected: &'a str) -> Result<&'a str, String> {
    if statement.is_none() {
        Err(format!("Expected {} statement, instead statement ended", expected))
    } else {
        Ok(statement.unwrap())
    }
}

fn parse_column_names<'a>(iterator: &mut dyn Iterator<Item=&'a str>) -> Result<Vec<&'a str>, String> {
    let mut expected_comma = false;
    let mut columns = Vec::<&'a str>::new();

    while let Some(mut statement) = iterator.next() {

        let is_first_comma = statement.starts_with(",");
        match (expected_comma, is_first_comma) {
            (true, true) => {
                statement = &statement[1..];
                if statement.len() == 0 {
                    expected_comma = false;
                    continue;
                }
            },
            (false, false) => {},
            (true, false) => return Err(format!("Expected statement, started with \",\", instead found \"{}\"", statement)),
            (false, true) => return Err(format!("Expected statement, not started with \",\", instead found \"{}\"", statement)),
        }

        let split_by_comma: Vec<&str> = statement.split(",").filter(|s| !s.eq(&"")).collect();

        for i in 0..split_by_comma.len()-1 {
            columns.push(split_by_comma.get(i).unwrap());
        }

        let last_split = split_by_comma.get(split_by_comma.len() - 1).unwrap();
        if last_split.ends_with(")") {
            columns.push(&last_split[..last_split.len()-1]);
            return Ok(columns);
        } else {
            columns.push(&last_split);
        }

        expected_comma = !statement.ends_with(",");
    }

    return Err(String::from("Failed to find closing bracket for column names"));
}

fn extract_open_bracket<'a>(statement: &Option<&'a str>) -> Result<Box<dyn Iterator<Item=&'a str> + 'a>, String> {

    match statement.as_ref() {
        Some(st) => {
            if st.starts_with("(") {
                if st.len() > 1 {
                    return Ok(Box::new(std::iter::once(&st[1..])));
                } else {
                    return Ok(Box::new(std::iter::empty()));
                }
            } else {
                return Err(format!("Expected statement, started with \"(\", instead found \"{}\"", st));
            }
        },
        None => return Err(format!("Expected statement, started with \"(\", instead statement ended")),
    }
}


fn parse_values<'a>(iterator: &mut dyn Iterator<Item=&'a str>) -> Result<Vec<&'a str>, String> {
    let mut expected_comma = false;
    let mut values = Vec::<&'a str>::new();

    let mut escaped_sequence = false;
    let mut varchar_sequence = false;

    while let Some(mut statement) = iterator.next() {

        let is_first_comma = statement.starts_with(",");
        match (expected_comma, is_first_comma) {
            (true, true) => {
                statement = &statement[1..];
                if statement.len() == 0 {
                    expected_comma = false;
                    continue;
                }
            },
            (false, false) => {},
            (true, false) => return Err(format!("Expected statement, started with \",\", instead found \"{}\"", statement)),
            (false, true) => return Err(format!("Expected statement, not started with \",\", instead found \"{}\"", statement)),
        }

        let chars: Vec<char> = statement.chars().collect();
        let mut current_sequence_index = 0usize;

        for mut i in 0..chars.len() {
            if varchar_sequence && chars[i] == '\\' && i != chars.len()-1 && chars[i+1] == '\'' {
                escaped_sequence = !escaped_sequence;
                i += 1;
                continue;
            }

            if chars[i] == '\'' {
                varchar_sequence = !varchar_sequence;
                continue;
            }

            if chars[i] == ',' && !varchar_sequence {
                values.push(&statement[current_sequence_index..i]);
                current_sequence_index = i+1;
            }

            if chars[i] == ')' && !escaped_sequence && !varchar_sequence {
                values.push(&statement[current_sequence_index..i]);
                return Ok(values);
            }
        }

        if current_sequence_index != chars.len() && !varchar_sequence {
            values.push(&statement[current_sequence_index..statement.len()]);
        }

        if !varchar_sequence {
            expected_comma = !statement.ends_with(",");
        }
    }
    Err(String::from("Expected statements to end with ), insted they ended without it"))
}

#[cfg(test)]
mod tests {


}