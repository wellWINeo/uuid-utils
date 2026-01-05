use clap::{Parser, Subcommand};

mod formatters;
mod generators;
mod subcommands;

#[derive(Parser)]
#[command(name = "uuid", about = "UUID utilities", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new UUID
    Gen {
        /// UUID version (v4 or v7)
        #[arg(default_value = "v4")]
        version: String,

        /// Output in uppercase
        #[arg(long)]
        uppercase: bool,

        /// Output in compact format (no hyphens)
        #[arg(long)]
        compact: bool,

        /// Output with 0x hex prefix
        #[arg(long)]
        hex: bool,
    },

    /// Format an existing UUID
    Fmt {
        /// UUID to format (reads from stdin if not provided)
        uuid: Option<String>,

        /// Output in canonical format (with hyphens)
        #[arg(long)]
        canonical: bool,

        /// Output in compact format (no hyphens)
        #[arg(long)]
        compact: bool,

        /// Output with 0x hex prefix
        #[arg(long)]
        hex: bool,

        /// Output in uppercase
        #[arg(long)]
        uppercase: bool,
    },

    /// Display information about a UUID
    Info {
        /// UUID to inspect
        uuid: String,
    },

    /// Return the nil UUID (00000000-0000-0000-0000-000000000000)
    Nil {
        /// Output in uppercase
        #[arg(long)]
        uppercase: bool,

        /// Output in compact format (no hyphens)
        #[arg(long)]
        compact: bool,

        /// Output with 0x hex prefix
        #[arg(long)]
        hex: bool,
    },

    /// Normalize a .NET System.Guid to RFC4122 format
    Normalize {
        /// UUID in .NET System.Guid byte order
        uuid: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gen {
            version,
            uppercase,
            compact,
            hex,
        } => {
            subcommands::generate::run(subcommands::generate::GenArgs {
                version,
                uppercase,
                compact,
                hex,
            });
        }
        Commands::Fmt {
            uuid,
            canonical: _,
            compact,
            hex,
            uppercase,
        } => {
            subcommands::fmt::run(subcommands::fmt::FmtArgs {
                uuid,
                compact,
                hex,
                uppercase,
            });
        }
        Commands::Info { uuid } => {
            subcommands::info::run(subcommands::info::InfoArgs { uuid });
        }
        Commands::Nil {
            uppercase,
            compact,
            hex,
        } => {
            subcommands::nil::run(subcommands::nil::NilArgs {
                uppercase,
                compact,
                hex,
            });
        }
        Commands::Normalize { uuid } => {
            subcommands::normalize::run(subcommands::normalize::NormalizeArgs { uuid });
        }
    }
}
