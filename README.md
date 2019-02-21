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

## Literals :
    - StringLiteral
    - NumericLiteral
    - BooleanLiteral)
    - NullLiteral
    - Infinity

## Todo :

- implement loc
- implement the other control flow expression
- implement interpretter for function and object
- add unit test for operator overloading
- add it test for interpretter
- add cli argument for pretty printer/interpret mode
- add repl mode
- add semantic documentation

## To fix :

- remote dirty copy with the Cell/RefCell type or a custom Box<> implementation
- implement a custom deserializer for JSLiteral or flatten the Expression struct

```
#include "c_datalib/databox.h"
#include <stdio.h>

void main() {
    databox x = {.data.num = 0};
    double *ptx = &(x.data.num);
    printf("%f", ptx);
    while (*ptx < 10) {
        *ptx+=1;
        printf("%f\n", *ptx);
    }

} 
```
