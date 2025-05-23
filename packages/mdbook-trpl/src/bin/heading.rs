use std::io;

use clap::{self, Parser, Subcommand};
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};

use mdbook_trpl::Heading;

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    if let Some(Command::Supports { renderer }) = cli.command {
        return if Heading.supports_renderer(&renderer) {
            Ok(())
        } else {
            Err(format!("Renderer '{renderer}' is unsupported"))
        };
    }

    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())
        .map_err(|e| format!("{e}"))?;
    let processed = Heading.run(&ctx, book).map_err(|e| format!("{e}"))?;
    serde_json::to_writer(io::stdout(), &processed).map_err(|e| format!("{e}"))
}

/// A simple preprocessor for semantic markup for code listings in _The Rust
/// Programming Language_.
#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Is the renderer supported?
    ///
    /// This supports the HTML
    Supports { renderer: String },
}
