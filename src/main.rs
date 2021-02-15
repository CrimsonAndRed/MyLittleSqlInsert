use std::env;

#[derive(Debug)]
struct ParsedStatement {
    table_name: String,
    value_pairs: Vec<(String, String)>
}

#[derive(Debug)]
struct InsertParser {
    statement: Vec<char>,
    index: usize
}

impl InsertParser {

    pub fn new<'a>(statement: &'a str) -> InsertParser {
        // Statement iterator
        let statement_vec = statement.chars().collect::<Vec<char>>();
        InsertParser {
            statement: statement_vec,
            index: 0usize
        }
    }

    pub fn parse(&mut self) -> Result<ParsedStatement, String> {
        self.required_keyword("insert")?;
        self.skip_whitespaces();
        self.required_keyword("into")?;
        self.skip_whitespaces();
        let mut table_name = self.required_to_be_some("table name")?.into_iter().collect::<String>();
        self.skip_whitespaces();
        // TODO safe indexing
        if self.statement[self.index] != '(' {
            table_name = format!("{} {}", table_name, self.required_to_be_some("table alias")?.into_iter().collect::<String>());
            self.skip_whitespaces();
        }
        self.required_keyword("(")?;
        let column_names = self.parse_column_names()?
            .into_iter()
            .map(|it| self.statement[it.0..it.1].iter().collect::<String>())
            .collect::<Vec<String>>();
        self.required_keyword(")")?;
        self.skip_whitespaces();
        self.required_keyword("values")?;
        self.skip_whitespaces();
        self.required_keyword("(")?;
        let values = self.parse_values()?
            .into_iter()
            .map(|it| self.statement[it.0..it.1].iter().collect::<String>())
            .collect::<Vec<String>>();

        Ok(ParsedStatement{
            table_name: table_name,
            value_pairs: column_names.into_iter().zip(values).collect()
        })
    }

    fn required_keyword(&mut self, keyword: &str) -> Result<(), String> {
        let statement_length = self.statement.len();

        for (j, ch) in keyword.chars().enumerate() {
            if self.index >= statement_length {
                return Err(format!("Expected symbol \"{}\" in \"{}\" statement at index \"{}\", instead statement ended", &ch, &keyword, &j));
            }
            let st = self.statement[self.index];
            match st.to_lowercase().next() {
                Some(lower) => {
                    if !lower.eq(&ch) {
                        return Err(format!("Expected symbol \"{}\" in \"{}\" statement at index \"{}\", instead got \"{}\"", &ch, &keyword, &j, &lower));
                    }
                },
                None => {
                    if !st.eq(&ch) {
                        return Err(format!("Expected symbol \"{}\" in \"{}\" statement at index \"{}\", instead got \"{}\"", &ch, &keyword, &j, &st));
                    }
                }
            }

            self.index += 1;
        }
        Ok(())
    }

    fn skip_whitespaces(&mut self) {
        let limit = self.statement.len();
        while self.index < limit {
            if self.statement[self.index].is_whitespace() {
                self.index += 1;
            } else {
                return;
            }
        }
    }

    fn required_to_be_some(&mut self, expected: &str) -> Result<&[char], String> {
        let start_index = self.index;
        let statement_length = self.statement.len();

        while self.index < statement_length {
            if self.statement[self.index].is_whitespace() {
                self.index += 1;
                break;
            } else {
                self.index += 1;
            }
        }
        if start_index == self.index {
            Err(format!("Expected {} statement, instead statement ended", expected))
        } else {
            Ok(&self.statement[start_index..self.index-1])
        }
    }

    fn parse_column_names(&mut self) -> Result<Vec<(usize, usize)>, String> {
        let mut expected_comma = false;
        let mut columns = Vec::<(usize, usize)>::new();
        let limit = self.statement.len();

        let mut word_start_index = None;

        // TODO Invalid SQL Insert queries still could be parsed, like "... column1,,,,,,column2 ..."
        // Also trailing whitespaces are added to output
        while self.index < limit {
            if self.statement[self.index] == ')' {
                if let Some(word_start) = word_start_index {
                    columns.push((word_start, self.index));
                }
                return Ok(columns);
            }
            if self.statement[self.index].is_whitespace() {
                if let Some(word_start) = word_start_index {
                    columns.push((word_start, self.index));
                    word_start_index = None;
                    expected_comma = true;
                }
                self.index += 1;
                continue;
            }
            if self.statement[self.index] == ',' {
                if let Some(word_start) = word_start_index {
                    columns.push((word_start, self.index));
                    word_start_index = None;
                }
                self.index += 1;
                expected_comma = false;
                continue;
            }
            // Regular character
            if expected_comma {
                return Err(format!("Expected statement, started with \",\", instead found \"{}\" at index \"{}\"", self.statement[self.index], &self.index))
            }
            if word_start_index.is_none() {
                word_start_index = Some(self.index);
            }
            self.index += 1;
        }

        return Err(String::from("Failed to find closing bracket for column names"));
    }


    fn parse_values<'a>(&mut self) -> Result<Vec<(usize, usize)>, String> {
        let mut expected_comma = false;
        let mut values = Vec::<(usize, usize)>::new();

        let limit = self.statement.len();

        let mut word_start_index = None;

        while self.index < limit {
            if self.statement[self.index] == ')' {
                if let Some(word_start) = word_start_index {
                    values.push((word_start, self.index));
                }
                return Ok(values);
            }
            if self.statement[self.index].is_whitespace() {
                if let Some(word_start) = word_start_index {
                    values.push((word_start, self.index));
                    word_start_index = None;
                    expected_comma = true;
                }
                self.index += 1;
                continue;
            }
            if self.statement[self.index] == ',' {
                if let Some(word_start) = word_start_index {
                    values.push((word_start, self.index));
                    word_start_index = None;
                }
                self.index += 1;
                expected_comma = false;
                continue;
            }
            // Regular character
            if expected_comma {
                return Err(format!("Expected statement, started with \",\", instead found \"{}\" at index \"{}\"", self.statement[self.index], &self.index))
            }
            if word_start_index.is_none() {
                word_start_index = Some(self.index);
            }
            self.index += 1;
        }

        Err(String::from("Expected statements to end with ), insted they ended without it"))
    }
}


fn main() -> Result<(), String> {
    let insert_string = env::args().nth(1).unwrap();

    let mut parser = InsertParser::new(&insert_string);
    let parsed = parser.parse()?;

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


#[cfg(test)]
mod tests {


}