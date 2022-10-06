# AlgorithmicPasswordGenerator

###### GNU Public Licence v3, 2022, Ruby-Dragon

Generates a memorable password using a simple algorithm and user input

## Install

This program supports Windows, Linux, and probably macOS.

Use Cargo on this repository:

    git clone https://github.com/Ruby-Dragon/AlgorithmicPasswordGenerator.git
    cd ./AlgorithmicPasswordGenerator
    cargo build --release

### Dependencies:

none

## Usage

This is a command line program, run from terminal with cargo: (compiles each time)

    cargo run

or if built, run from terminal with: (Recommended)

    ./AlgorithmicPasswordGenerator

*do not use ./ on Windows*

#### In Program:

In the program you will be prompted for sets of words. Each set will be run through one of 3 algorithms:

1. No Vowels - Removes all vowels in all words input, ex. "buy stuff" -> "bystff"
2. Abbreviate - Removes all letters but capitals, ex. "YouTube" -> "YT"
3. Swap - Swaps the first and last letters of the first word and discards all other words, ex. "hello world" -> "oellh"

When all word sets you wish to be present in the password have been input, use **-d** to exit. 
Your finished password will be output. This password is ***NOT SAVED*** so please make sure to remember it or write it down somewhere.

Note: Do NOT use any non-ascii characters! Unicode characters such as "ω" are not supported for use in this program and will cause an error.

## Licence


This software uses the GPL licence. Read the terms before using the source code.

###### ALL DERIVATIVE WORKS MUST BE GPL v3 LICENCED AS WELL, AND MUST KEEP ALL COPYRIGHT NOTICES IN CODE. See the LICENCE for more information.
