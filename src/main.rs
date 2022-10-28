use clap::Parser;

/// This is a doc comment with info about my cli tool.
#[derive(Parser)]
struct Cli {
    /// this is a doc comment with info about port argument
    #[arg(short, long, default_value_t = 8080)]
    // if long and short are omitted the cli uses arguments without flags.
    // if short and long are present, option flags must be used to pass options.  
    // clap::Parser also supports default values with default_value_t
    port: u16,
}


fn main() {
    let cli = Cli::parse();
    println!("PORT = {}", cli.port);
}
