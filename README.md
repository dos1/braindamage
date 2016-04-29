# BrainDamage
A simple engine for writing games using Brainfuck, written in Rust.

## Brainfuck details
As some details of Brainfuck are not defined in the language, this is the set of rules assumed in BrainDamage:
* unlimited number of 32-bit signed memory cells
* no negative indexes
* cells wrap on overflow
* EOF returns 0
* IO is blocking and buffered
* no newline translation

Also, until *Read::chars()* in Rust is marked as stable, multibyte characters received from input are split per byte.

---
BrainDamage is distributed on WTFPL, Version 2

by dos - Sebastian Krzyszkowiak
