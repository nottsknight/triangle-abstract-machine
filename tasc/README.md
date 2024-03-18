# TAM Assembly Compiler

This crate provides an executable `tasc` for compiling a 
simple assembly language for the Triangle Abstract Machine. Its
syntax differs slightly from that given in Watt & Brown, so as 
to make it appear closer to the x86 syntax familiar to many.

## Usage
The executable expects one argument that is the name of the 
assembly file to compile. It has one option `-o` which specifies
the name of the binary file to create. It defaults to `a.out`,
as is tradition.

## Assembly syntax 
All instructions are lowercase. An instruction begins with a mnemonic,
followed by its arguments. If an instruction accepts two arguments 
they are separated by a comma.

### Number format
Literal numbers may be given in either decimal or hexadecimal form. 
Hexadecimal numbers should be preceded by `0x`.

### Address format
Most instructions require an address as an argument. An address is 
surrounded by square brackets and consists of a register name and an 
offset.

Register names are the same as in Watt & Brown, but lower case. An offset 
is a *decimal* number with a `+` or `-` operator preceding.

### Primitive procedures
Primitive procedures can be called by name rather than calculating the offset
from the `pb` register. Note that the multiplication primitive is called `mul`
rather than `mult`.

### Example
The following program requests two numbers from the user and prints 
the larger of the two numbers. Other example programs are available in 
the [tests](./tests) directory.

```
push    2 
loada   [sb+0]
call    getint
loada   [sb+1]
call    getint 
load    1, [sb+0]
load    1, [sb+1]
call    ge 
jumpif  0, [cb+12]
load    1, [sb+0]
call    putint
jump    [cb+14]
load    1, [sb+1]
call    putint 
call    puteol
halt
```
