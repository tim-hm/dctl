use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct ArgsList {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Theme(ThemeCommand),
}

#[derive(Debug, Subcommand)]
pub enum ThemeCommand {
    #[command()]
    Toggle,
}
