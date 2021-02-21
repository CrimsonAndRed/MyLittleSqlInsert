lalrpop_mod!(pub sqlinsert); // synthesized by LALRPOP

pub fn parse<'a>(insert: &'a str) -> Result<SqlInsert<'a>, String>  {
    sqlinsert::SqlInsertParser::new().parse(insert)
        .map_err(|e| format!("{}", e))
}

pub struct SqlInsert<'a> {
    pub table_name: &'a str,
    pub alias: Option<&'a str>,
    pub columns: Vec<&'a str>,
    pub values: Vec<String>
}

impl<'a> SqlInsert<'a> {

    pub fn new<'b>(table_name: &'b str, alias: Option<&'b str>, columns: Vec<&'b str>, values: Vec<String>) -> SqlInsert<'b> {
        SqlInsert {
            table_name: table_name,
            alias: alias,
            columns: columns,
            values: values
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_test1() {
        let parsed = sqlinsert::SqlInsertParser::new().parse("insert into test () values ()");
        assert!(parsed.is_ok());
        let statement = parsed.unwrap();

        // let vec: Vec<&'static str> = Vec::<&'static str>::new();
        assert_eq!("test", statement.table_name);
        assert_eq!(None, statement.alias);
        assert_eq!(Vec::<&'static str>::new(), statement.columns);
        assert_eq!(Vec::<String>::new(), statement.values);
    }

    #[test]
    fn correct_test2() {
        let parsed = sqlinsert::SqlInsertParser::new().parse("INSERT INTO TEST (COL1, COL2) VALUES (VAL1, VAL2)");
        assert!(parsed.is_ok());
        let statement = parsed.unwrap();
        assert_eq!("TEST", statement.table_name);
        assert_eq!(None, statement.alias);
        assert_eq!(vec!["COL1", "COL2"], statement.columns);
        assert_eq!(vec!["VAL1".to_string(), "VAL2".to_string()], statement.values);
    }

    #[test]
    fn correct_test3() {
        let parsed = sqlinsert::SqlInsertParser::new().parse("InSeRt      InTo   TeSt   ( CoL1 , CoL2 ) VaLuEs (vAl1, VaL2) ");
        assert!(parsed.is_ok());
        let statement = parsed.unwrap();
        assert_eq!("TeSt", statement.table_name);
        assert_eq!(None, statement.alias);
        assert_eq!(vec!["CoL1", "CoL2"], statement.columns);
        assert_eq!(vec!["vAl1".to_string(), "VaL2".to_string()], statement.values);
    }

    #[test]
    fn correct_test4() {
        let parsed = sqlinsert::SqlInsertParser::new().parse("InSeRt      InTo TeSt alias (CoL1, Col2) VaLues (Val1, Val2);");
        assert!(parsed.is_ok());
        let statement = parsed.unwrap();
        assert_eq!("TeSt", statement.table_name);
        assert_eq!(Some("alias"), statement.alias);
        assert_eq!(vec!["CoL1", "Col2"], statement.columns);
        assert_eq!(vec!["Val1".to_string(), "Val2".to_string()], statement.values);
    }

    #[test]
    fn correct_test5() {
        let parsed = sqlinsert::SqlInsertParser::new().parse("InSeRt      InTo TeSt alias (CoL1, Col2) VaLues ('222222', Val2);");
        assert!(parsed.is_ok());
        let statement = parsed.unwrap();
        assert_eq!("TeSt", statement.table_name);
        assert_eq!(Some("alias"), statement.alias);
        assert_eq!(vec!["CoL1", "Col2"], statement.columns);
        assert_eq!(vec!["'222222'".to_string(), "Val2".to_string()], statement.values);
    }

    #[test]
    fn correct_test6() {
        let parsed = sqlinsert::SqlInsertParser::new().parse("InSeRt      InTo TeSt alias (CoL1, Col2) VaLues (to_date('11.02.2021', 'DD.MM.YYYY'), null);");
        assert!(parsed.is_ok());
        let statement = parsed.unwrap();
        assert_eq!("TeSt", statement.table_name);
        assert_eq!(Some("alias"), statement.alias);
        assert_eq!(vec!["CoL1", "Col2"], statement.columns);
        assert_eq!(vec!["to_date('11.02.2021', 'DD.MM.YYYY')".to_string(), "null".to_string()], statement.values);
    }

    #[test]
    fn incorrect_test1() {
        let parsed = sqlinsert::SqlInsertParser::new().parse("");
        assert!(parsed.is_err());
    }

    #[test]
    fn incorrect_test2() {
        let parsed = sqlinsert::SqlInsertParser::new().parse("insrt into test");
        assert!(parsed.is_err());
    }

    #[test]
    fn incorrect_test3() {
        let parsed = sqlinsert::SqlInsertParser::new().parse("insert ino test");
        assert!(parsed.is_err());
    }

    #[test]
    fn incorrect_test4() {
        let parsed = sqlinsert::SqlInsertParser::new().parse("insert into test test test");
        assert!(parsed.is_err());
    }

    #[test]
    fn incorrect_test5() {
        let parsed = sqlinsert::SqlInsertParser::new().parse("insert into test test (test ");
        assert!(parsed.is_err());
    }

    #[test]
    fn incorrect_test6() {
        let parsed = sqlinsert::SqlInsertParser::new().parse("insert into test test (test values");
        assert!(parsed.is_err());
    }
}