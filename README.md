# BrainDamage
A simple engine for writing games using Brainfuck, written in Rust.

## Brainfuck constraints
As some details of Brainfuck are not defined in the language, this is the set of rules assumed in BrainDamage:
* 100000 memory cells
* no negative indexes
* 32-bit signed wrapping cells
* EOF writes 0 to selected cell

Also, until *Read::chars()* in Rust is marked as stable, multibyte characters received from input are split per byte.

---
BrainDamage is distributed on WTFPL, Version 2

by dos - Sebastian Krzyszkowiak
