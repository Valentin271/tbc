use std::path::PathBuf;

use argh::FromArgs;

/// A tinyBASIC compiler
#[derive(FromArgs)]
pub struct Cli {
    /// the file to compile
    #[argh(positional)]
    pub file: PathBuf,
    /// run the program if compiled successfully
    #[argh(switch, short = 'r')]
    pub run: bool,
}
