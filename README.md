# Compil TP 1

### Dependencies
- rustc 1.34.0-nightly
- cargo
- gcc 
- babylon
- GNU indent (optional)


### build

```cargo build --release```

### run

The binary file is located in `target/release`, run it like so : ```./rjsc ${flag} ${args}```
If you are lost :`rjsc --help`. 

### test

To test the compiler with the provided examples run `./sandbox.sh`


## C compiler Todo :

- implement nested function

## Current status


| Examples                  | C compile           | ASM compile | Interpret |
| ---------------           |:-------------------:|:-----------:|:---------:|
| 01-expressions.js         | OK                  | KO          | KO        |
| 02-declarations.js        | OK                  | KO          | KO        |
| 03-while.js               | OK                  | KO          | KO        |
| 04-if-while.js            | OK                  | KO          | KO        |
| 05-fors.js                | OK                  | KO          | KO        |
| 06-while-break.js         | OK                  | KO          | KO        |
| 11-func.js                | OK                  | KO          | KO        |
| 12-fact.js                | OK                  | KO          | KO        |
| 13-switch.js              | KO                  | KO          | KO        |
| 14-obj.js                 | OK                  | KO          | KO        |
| 15-new.js                 | KO                  | KO          | KO        |
| 16-funcs.js               | KO                  | KO          | KO        |
| 17-double-parenth-func.js | KO                  | KO          | KO        |
| 18-func-in-func.js        | KO                  | KO          | KO        |
