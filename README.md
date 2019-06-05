# DataGen
> An easy to use tool to generate fake/dummy data in bulk and export it as Avro, Parquet or directly into your database as tables.


[![Build Status](https://travis-ci.com/arunma/datagen.svg?branch=master)](https://travis-ci.com/arunma/datagen)



DataGen is a command line application written in [Rust][] that generates dummy data for provides a means of interacting with the social Web from your personal
desktop.

## Features

 * Export Data as Files
   * [x] CSV
   * [x] Avro
   * [ ] Parquet
 * Export Data into Database
   * [ ] Postgres
   * [ ] MySQL
 * [ ] Generate unique records by respecting the primary key attribute
 * [ ] Generate multiple datasets with PrimaryKey/ForeignKey
 * [ ] Support Richer types - Date, Map, Arrays, Nested Records
 
## Installation

At the moment, the installation is done only through [Cargo](https://www.rust-lang.org/tools/install).  Please install Cargo by following the instructions from https://www.rust-lang.org/tools/install.

Once cargo is installed, you could pull the binary from [crates.io](https://crates.io/crates/datagen) using : 

```bash

cargo install datagen

```


> Note: The binary would have been placed in your `<HOME_DIR>/.cargo/bin/` which the Cargo installation would have placed in your PATH.  If not, please add it to your PATH.

<!--
OS X & Linux:

```sh
npm install my-crazy-module --save
```

Windows:

```sh
edit autoexec.bat
```
-->

## Usage example

##### CSV 

```$bash

datagen csv "<output_dir>/output.csv" "<schema_yaml_dir>/schema.yaml" 100 "^"

```

[![asciicast](https://asciinema.org/a/249996.png)](https://asciinema.org/a/249996)


##### Avro

```$bash

datagen avro "<output_dir>/output.avro" "<schema_yaml_dir>/schema_simple.yaml" 100

```

[![asciicast](https://asciinema.org/a/249989.png)](https://asciinema.org/a/249989)

<!--
A few motivating and useful examples of how your product can be used. Spice this up with code blocks and potentially more screenshots.

_For more examples and usage, please refer to the [Wiki][wiki]._
-->

## Schema YAML

An example for the schema YAML is located at `<PROJECT_ROOT>/test_data/schema_simple.yaml`

## Development setup

1. Clone the [repo](https://github.com/arunma/datagen.git)
2. Run `cargo build`
3. Run `cargo test -- --color always --nocapture`
4. Run program (& Profit!)

##### CSV
```bash
cargo run -- "csv" "<output_dir>/output.csv" "<schema_yaml_dir>/schema.yaml" 100 ";"
```

##### Avro
```bash
cargo run -- "avro" "<output_dir>/output.avro" "<schema_yaml_dir>/schema_simple.yaml" 100
``` 



<!--
Describe how to install all development dependencies and how to run an automated test-suite of some kind. Potentially do this for multiple platforms.

```sh
make install
npm test
```
-->

## Release History

* 0.1.0
    * Support for CSV (no headers)
    * Support for Avro (primitive types)
    
* 0.1.1
    * Support for custom delimiters for CSV

## Meta

Arun Manivannan – [@arunma](https://twitter.com/arunma) – arun@arunma.com

Distributed under the MIT license. See ``LICENSE`` for more information.

[https://github.com/arunma/datagen](https://github.com/arunma/datagen)

## Contributing

You want to help out? _Awesome_! 

1. This is my first Rust project.  If you are an experienced Rust programmer, I can't thank enough for doing a code review. 
2. If you are interested in adding new **sinks** to the project/report bugs/add features/add docs, thank you in advance.  Your efforts are very much appreciated.   

<!-- Markdown link & img dfn's 

[wiki]: https://github.com/yourname/yourproject/wiki-->