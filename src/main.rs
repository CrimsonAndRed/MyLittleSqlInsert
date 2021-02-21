#[macro_use] extern crate lalrpop_util;

use std::env;
mod lalrpop;


fn main() -> Result<(), String> {
    let insert_string = env::args().nth(1).unwrap();
    let parsed = lalrpop::parse(&insert_string)?;


    let mut pad_length = 0;
    for column_name in &parsed.columns {
        pad_length = usize::max(pad_length, column_name.len());
    }

    if let Some(alias) = parsed.alias {
        println!("INSERT INTO {} {}:", parsed.table_name, alias);
    } else {
        println!("INSERT INTO {}:", parsed.table_name);
    }


    for i in 0..parsed.columns.len() {
        println!("  {:<pad$} : {}", parsed.columns[i], parsed.values[i], pad = pad_length);
    }

    Ok(())
}