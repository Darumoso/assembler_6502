# 6502 Assembler
## Intallation

### Dependencies
In case rust isn't installed, it can be installed with:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Building
First clone the repo:
```
git clone https://github.com/Darumoso/assembler_6502 && cd assembler_6502
```

Then build with:
```
cargo build --release
```

### Runing the parser
Once the project is succesfully builded, a binary can be found at:

```
cd target/release
```

Then make it executable
```
chmod +x ./assembler_6502
```
To run it, enter:
```
./assembler_6502
```
There will be 3 succesful outputs, and 1 error output, the error output is expected to
fail


# Code examples
## Successfull outputs
The next are the correct inputs received by the parser

### Example 1
```6502
LDX #$00
LDY #$00
firstloop:
    TXA
    STA $0200,Y
    PHA
    INX
    INY
    CPY #$10
    BNE firstloop
secondloop:
    PLA
    STA $0200,Y
    INY
    CPY #$20
    BNE secondloop
```

### Example 2
```6502
LDX #$08
decrement:
    DEX
    STX $0200
    CPX #$03
    BNE decrement
    STX $0201
    BRK
```
### Example 3
```6502
LDX #$01
LDA #$aa
STA $a0,X
INX
STA $a0,X
```

## Error output
The next is the error input received by the parser
### Example 1
```6502
LDY #$00
firstloop
    STA $0200,Y
```

### Example 2
This error isn't reachable by the parser, to show it line 150 on main file must
be commented and lines 168-169 must be uncommented
```6502
LDY #$00
firstloop:
    STA $0200,Y
```
