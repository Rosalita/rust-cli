use clap::Parser;
use log::{debug, error, info, trace, warn};

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
    env_logger::init();
    trace!("cli is running");
    debug!("about to parse args");
    let cli = Cli::parse();
    info!("args have been parsed");
    warn!("cli port is {}", cli.port);
    print!("port is {}", cli.port);
    error!("something unexpected");
    print!("{}", answer());
}

fn answer() -> i32{
    return 42;
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_answer(){
        assert_eq!(answer(), 42);
    } 
}
