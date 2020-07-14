# JSON example for Sana

This is a simple example of using Sana as a JSON lexer.

## Usage

```
cargo run --example json < example.json
```

It will output the tokens in the `{token_name} at {start}..{end}` format.

Example:

```bash
$ echo '{"x":1}' | cargo run --example json
LBrace at 0..1
String at 1..4
Colon at 4..5
Number at 5..6
RBrace at 6..7
Whitespace at 7..8
```
