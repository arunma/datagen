# DataGen
> An easy to use tool to generate fake/dummy data in bulk and export it as Avro, CSV, Json or directly into your database as tables (coming soon!).


[![Build Status](https://travis-ci.com/arunma/datagen.svg?branch=master)](https://travis-ci.com/arunma/datagen)



DataGen is a command line application written in [Rust](https://www.rust-lang.org/) that generates dummy data for provides a means of interacting with the social Web from your personal
desktop.

## Features

 * Export Data as Files
   * [x] CSV
   * [x] Avro
   * [x] Json
   * [ ] Parquet
 * Export Data into Database
   * [ ] Postgres
   * [ ] MySQL
 * [x] Supports Int, Long, Double, Float, String, Date, DateTime
 * [x] Supports `one_of` to generate random values from a list
 * [x] Supports `min` and `max` for numeric and date fields
 * [x] Supports `mean` and `std` for numeric fields
 * [x] Supports custom date formatting for Date and DateTime datatypes
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


##### Json

```$bash

datagen json "<output_dir>/output.json" "<schema_yaml_dir>/all_examples.yaml" 100

```

<!--
A few motivating and useful examples of how your product can be used. Spice this up with code blocks and potentially more screenshots.

_For more examples and usage, please refer to the [Wiki][wiki]._
-->

## Schema YAML

    ---
    name: person_schema
    dataset:
      name: person_table
      columns:
        - {name: id, not_null: false, dtype: int}
        - {name: name, dtype: name}
        - {name: age, dtype: age}
        - {name: adult, default: 'false', dtype: boolean}
        - {name: gender, dtype: string, one_of: ["M", "F"]}
        - {name: dob, dtype: "date", min: "01/01/1950" , max: "03/01/2014", format: "%d/%m/%Y"}
        - {name: event_date, dtype: "datetime", min: "2014-11-28 12:00:09" , max: "2014-11-30 12:00:09", format: "%Y-%m-%d %H:%M:%S"}
        - {name: score, dtype: "int", mean: 1.00, std: 0.36}
        - {name: distance, dtype: "int", min: 19000, max: 221377}
        - {name: weight, dtype: "float", min: 1.00, max: 500.00}
    
Date format specifiers could be sourced from : https://docs.rs/chrono/0.4.9/chrono/format/strftime/index.html#specifiers
    
An example for the schema YAML is located at `<PROJECT_ROOT>/test_data/schema_options.yaml`


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

##### Json
```bash
cargo run -- "json" "<output_dir>/output.json" "<schema_yaml_dir>/schema.yaml" 100
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
    
* 0.1.3
    * Json support added
    * Support for semantic types (name, date, latitude, phone etc)
* 0.1.4
     * Supports one_of eg. 
        ```
                - {name: "day_of_week", dtype: "string", one_of:["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"]} 
        ```
     * Support for min and max for numeric columns 
        ```
                - {name: "age", dtype: "int", min: 1 , max: 130}
        ```
     * Support for Date and Datetime (along with min and max)
        ```
               - {name: "event_time", dtype: "datetime", min: "2014-11-28 12:00:09" , max: "2014-11-30 12:00:09", format: "%Y-%m-%d %H:%M:%S"}
               - {name: "dob", dtype: "date", min: "01/01/1920" , max: "03/01/2019", format: "%d/%m/%Y"}
        ```
     
     * Support for semantic types (name, date, latitude, phone etc) 

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