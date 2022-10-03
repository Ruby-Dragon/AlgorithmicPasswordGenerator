# AlgorithmicPasswordGenerator
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

This is a command line program, run from shell with cargo:

    cargo run

or if built:

    ./AlgorithmicPasswordGenerator

#### In Program:

In the program you will be prompted for sets of words. Each set will be run through one of 3 algorithms:

1. No Vowels - Removes all vowels in all words input
2. Abbreviate - Removes all letters but capitals, ex. "YouTube" -> "YT"
3. Swap - Swaps the first and last letters of the first word and discards all other words, ex. "hello world" -> "oellh"

When all word sets you wish to be present in the password have been input, use **-d** to exit. 
Your finished password will be output. This password is ***NOT SAVED*** so please make sure to remember it or write it down somewhere.