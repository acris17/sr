# Sr
Search from a variety of Search Engines using the command line


## Purpose
I've always found it slow to search for something using a particular search engine. For example, if I wanted to look for a particular crate in crates.io I would have to google crates.io, click the search box, then enter my query. Since I live in the command line this took too long. I thus I needed a way to perform a query on multiple search engines using one super command.

## Environment
* **MacOS** - version 10.13.6
* **Rust** - version 1.27.2

## Installation
1. Make sure you have the latest rust tools installed
2. Clone or download the repo
3. `$ cd sr`
4. `$ cargo build --release`
5. Binary is located at `./target/release/sr`
6. Place binary in path (look up `bash path` if you don't understand)

## Usage
* Usage: sr [ENGINE] [QUERY]...
* Help: `sr --help`
* Engine = The search engine to use
* Query = the query provided to the search engine

## Examples
* The following are all the supported search engines
* wikipedia: `sr wiki topology`
* google: `sr go hello world`
* rust: `sr rust std::env`
* cratesio: `sr crates clap`
* youtube: `sr youtube never gonna give you up`
* dictionary: `sr dict tomato`

## Design
The design is simple. The CLI recieves both the engine requested and the query as a string. The app then chooses a search engine function based on those inputs. This makes the app modular: I can add as many search engines as I want without technical debt. 

## Notes
This app is not portable and has been designed to work on MacOS. Unportable parts include the `dict` search engine and the way the app opens safari to execute the search query.

## Future
I want to add a `list` engine which lists the currently supported search engines. 