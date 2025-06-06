use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a Hoon file using hoonc
    Compile {
        /// Entry Hoon file
        entry: String,
        /// Directory with dependencies
        #[arg(long, default_value = "hoon")]
        deps: String,
        /// Output jam path
        #[arg(long)]
        output: Option<String>,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Compile { entry, deps, output } => {
            let jam = hoonc::build_jam(&entry, deps.into(), output.clone().map(Into::into), false, true)
                .await
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            println!("compiled {} -> {} bytes", entry, jam.len());
        }
    }
    Ok(())
}
