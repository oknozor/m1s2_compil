# Compil TP 1

# Prérequis
- rustc 1.32.0-nightly (6bfb46e4a 2018-11-26)
- cargo
## Usage

### build
```cargo build```

### run
- with cargo
```cargo run ${path_to_ast_json_file}```
- with bin
```./target/debug/jsparser ${path_to_ast_json_file}```

## TODO

- implement debbuging
- implement the other control flow expression
- implement interpretter for function and object
- add unit test for operator overloading
- add it test for interpretter
- add cli argument for pretty printer/interpret mode
- add repl mode
- add semantic documentation