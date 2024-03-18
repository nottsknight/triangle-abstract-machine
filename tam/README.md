# TAM emulator

This crate provides an implementation of the Triangle Abstract Machine.

The executable expects a single mandatory argument which is the binary
file to run. It also accepts one of two mutually exclusive options:

- `-t/--trace` will print the state of the stack after each instruction
- `-d/--disassemble` will print a dissasembly of the specified binary 
  instead of running it
