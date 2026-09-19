# BFPPC - Standalone Linux x64 Brainfuck++ Compiler

## Dependecies:
* clap     - For arguments parsing
* logos    - For fast lexing
* iced-x86 - For assembly generation
* object   - For creating elf files

### Example usages:
```sh
./bfppc -i hello.bf -o hello
```

### With optimizer:
```sh
./bfppc -i hello.bf -o hello --opt
```

### New instruction (Extension of original brainfuck)
#### 
```
*<command><count> Allow to generate multiple instructions

*+3 will generate +++
*<5 will generate <<<<<
*+'H'. will generate 72 pluses (Same as ASCII code of 'H')
```