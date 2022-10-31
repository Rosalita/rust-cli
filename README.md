# rust-cli

## Standard Library Env Args

The Rust Standard library contains `std::env::args()`
This returns an iterator of arguments and this can be used to get raw cli args.
e.g. `let all_args = std::env::args();` where all_args will be of type `Args`.

The first entry in this iterator will always be the name of the program.
The second entry can be captured with `let maybe_second_arg = std::env::args().nth(1);`
This return an `Option<String>` which can either be some or none.

However making this call this with `expect` will force it to always return some arg
as `expect` will force the code to panic with the expected string if it is none.
`let second_arg = std::env::args().nth(1).expect("no second arg given");`

Using the standard library is a fast way to implement a cli which only takes a single argument.

However a good cli tool will allow users to pass flags which can be long e.g `--port 1234` 
or short e.g. `-p 1234`. The standard library env args does not support flags.

## Crates, modules, items and attributes.

`Clap` is a Cargo crate. One of the things the `Clap` crate contains is a `Parser` that will
populate a struct with Cli arguments. `Parser` is a `module` inside the `Clap` crate. 

Crates use `::` to express module paths e.g. `crate::module`

`derive` is an `attribute`. `attributes` are either `inner` or `outer`.
* An `inner` attribute looks like `#![something]`
* An `outer` attribute looks like `#[something]`

The [docs](https://doc.rust-lang.org/reference/attributes.html#meta-item-attribute-syntax) say that this 
syntax originates from C#.

`derive` specifies a list of `traits` to process. 


## Clap Cli Parser

The docs for `clap` are at https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

Using `clap::Parser`, `Parser` is a `trait` of `clap` it's docs are at
https://docs.rs/clap/latest/clap/trait.Parser.html

So to use the `Parser` module in the `Clap` crate firstly the `Cargo.toml` file must specify
a dependency on the `Clap` crate. This is done in the `[Dependencies]` section using
`clap = {version = "4.0", features = ["derive"]}`

From the documentation for `clap` it can either use attributes like:

```
#[derive(Parser)]
#[command(about = "I am a program and I work, just pass `-h`", long_about = None)]
struct Foo {
    #[arg(short, help = "Pass `-h` and you'll see me!")]
    bar: String,
}
```

or, for convenience, it can use doc comments which are `///` to provide the same information:

```
#[derive(Parser)]
/// I am a program and I work, just pass `-h`
struct Foo {
    /// Pass `-h` and you'll see me!
    bar: String,
}
```

## Logging

There are two parts to logging in rust:
* The fascade, this lives in the crate `https://crates.io/crates/log` with docs at `https://docs.rs/log/latest/log/`.
The fascade is the logging API that Rust code can use with actual logging implementation abstracted. The reason for this
is that if a library uses the fascade, a consumer of the library can then select the most appropriate logging implementation.

* A logging implementation which is compatible with the fascade. For Cli tools `env_logger` is a good choice, 
see `https://docs.rs/env_logger/0.9.1/env_logger/`. By default `env_logger` writes to `stderr`, but can be configured to write to `stdout`.
By default all logging is disabled except for the `error` level. Logging is controlled by the `RUST_LOG` env var.

To use the fascade, add `log` to Cargo dependencies, then `use log::{trace, debug, info, warn, error};` and call the fascade API using
macros like `trace!("foo");`, `debug!("bar");` and `error!("baz")`. Remember that if there is no logging implementation, the fascade
will fall back to a `noop` implementation that ignores all log messages.

To start using the `env_logger` logger, add it to Cargo dependencies, then initialise a logger with `env_logger::init();`
it's documenation is at `https://docs.rs/env_logger/0.9.1/env_logger/#`. `env_logger` also contains a builder which can be used to 
change the default log level in code rather than using the `RUST_LOG` env var.

## Building and Running

`cargo build` generates the executable and outputs it to the `target` directory.
The executable file will be either in `target/debug` or `target/release` depending 
on which configuration was build.

Running executable with --help shows the doc comments with usage, arguments and options.

To run specifying an option
 * long form: `./target/debug/rust-cli.exe --port 1234` 
 * short form: `./target/debug/rust-cli.exe -p 1234`

By default only error logs will be shown, to see different logs set the `RUST_LOG` env var as appropriate.
To see all logs set the level to trace `export RUST_LOG=trace`.
