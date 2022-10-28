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

## Building and Running

`cargo build` generates the executable and outputs it to the `target` directory.
The executable file will be either in `target/debug` or `target/release` depending 
on which configuration was build.

Running executable with --help shows the doc comments with usage, arguments and options.

To run specifying an option
 * long form: `./target/debug/rust-httpserver.exe --port 1234` 
 * short form: `./target/debug/rust-httpserver.exe -p 1234`
