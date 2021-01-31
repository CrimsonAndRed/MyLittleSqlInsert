This project designed to prettify SQL insert statements.

# Supported formats
At this time this tool partially supports Oracle INSERT statements.
![Alt text](oracle.jpg?raw=true "Supported insert structure")

# Running
To run tool from source code:
```
cargo run -- '<your insert statement>'
```

Example:
```
cargo run -- 'insert into test_table (col1, col2, col3) values (1, 2, 3)'
```