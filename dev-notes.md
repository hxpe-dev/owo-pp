## Steps to add a new keyword.
1. Add the keyword into the `tokens.rs` file.
2. Implement its parsing in the `parser.rs` file.
3. Don't forget to add the new TokenType in the `if` statement containing all keywords token types in the `parse_function_declaration` function.
4. Implement its logic in the `interpreter.rs` file.