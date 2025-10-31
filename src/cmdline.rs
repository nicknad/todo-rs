use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "todo")]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    // insert todo item
    #[command(arg_required_else_help = true)]
    Insert {
        item: String,
        descritpion: Option<String>,
    },
    // marks item complete
    #[command(arg_required_else_help = true)]
    Complete {
        item: String,
    },
    // removes item from list
    #[command(arg_required_else_help = true)]
    Delete {
        item: String,
    },
    List {
        #[arg(short, long, default_value_t = false)]
        all: bool,
    },
    Clear,
}
