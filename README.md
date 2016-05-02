# BrainDamage [![Build Status](https://travis-ci.org/dos1/braindamage.svg?branch=master)](https://travis-ci.org/dos1/braindamage)
A simple engine for writing games using Brainfuck, written in Rust.

## Brainfuck details
As some details of Brainfuck are not defined in the language, this is the set of rules assumed in BrainDamage:
* unlimited number of 32-bit signed memory cells
* no negative indexes
* cells wrap on overflow
* EOF returns 0
* standard IO is blocking and buffered
* no newline translation

---
BrainDamage is distributed on WTFPL, Version 2

by dos - Sebastian Krzyszkowiak
