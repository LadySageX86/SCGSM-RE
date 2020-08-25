# SCGSM Rust Edition v0.1.2
## Developed by Spencer Smith (spenny@geniuspiece.com)
### Last updated 25 August 2020

*SCGSM* is a very simple specification for saving and managing game data. 

## Usage

This library is available through crates.io! To add it to your project, simply add the following line underneath the dependencies tag in your *Cargo.toml* file:

`scgsm = "0.1.2"`

## Syntax
The structure of an *SCGSM* file is as follows:

```
FLAG_A|1
FLAG_B|2
FLAG_C|3
```

The pipe operator (|) takes two parameters, a flag key on the left, and a flag value on the right. Flag values can be any type, and they will be stored as a `String` variable in the actual flag object, meaning that it's up to you to typecast values when applying them to variables in the Rust code. 

There must be only one key/value pair per line, and there must be no spaces between the parameters and pipe. 

## Changes from original version
Project renamed to avoid conflicts with another SCFS.

In the original C++ implementation of *SCGSM*, there was a function called `sc::GSM::parse()` that would read the *.scgs* file line by line and split it into separate strings. This function was unnecessary in the Rust Edition and all that functionality has been condensed into the `SCGSM::load_flags()` function instead.

Originally *.scgs* files would end with the line "ENDFLAGS", however this was decided to be redundant and thus removed from this version of the specification.

## Glossary
`pub struct Flag` - Structure which defines all of the in-game flags. Has two member variables, `pub key: String` and `pub value: String`.

`pub fn Flag::init(k: &str, v: &str) -> Flag` - Function that creates a new `Flag` with `key` k and `value` v.

`pub struct SCGSM` - Structure which defines objects and functions for saving, loading, and managing game data. Has one member variable, `pub flags: Vec<Flag>`, which stores all of the flags in memory.

`pub fn SCGSM::init() -> SCGSM` - Function that creates a new `SCGSM` variable with an empty vector.

`pub fn SCGSM::add_flag(&mut self, k: &str, v: &str)` - This function creates a new `Flag` object `f` and then calls `self.flags.push(f)`.

`pub fn SCGSM::load_flags(&mut self, sf: &str)` - This function reads the output of file *sf* line by line and uses a string splitting algorithm to create new flags for each line and add them to the `flags` vector. 

`pub fn SCGSM::save_flags(&mut self, sf: &str) -> std::io::Result<()>` - This function writes the output of `SCGSM::viewFlags()` to file *sf*.

`pub fn SCGSM::update_flag(&mut self, k: &str, v: &str)` - This function modifies the value of `Flag` objects by iterating through the `flags` list to find the key that matches `k` and changing the value of that flag to `v`. If the flag key cannot be found, the function will fail. 

`pub fn SCGSM::view_flags(&mut self) -> String` - This function returns the entire `flags` list as a `String` object. 
