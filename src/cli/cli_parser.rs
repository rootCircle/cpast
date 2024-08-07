use std::io;

use clap::{Command, CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::{generate, Generator, Shell};

const DEFAULT_ITERATIONS_COUNT: usize = 5;

#[derive(Parser)] // requires `derive` feature
#[command(name = "cpast", version, author, about, long_about = None)]
#[command(bin_name = "cpast")]
pub struct CpastCli {
    /// Generate Shell Completions
    #[arg(long = "completions", value_enum)]
    completions: Option<Shell>,
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand)] // requires `derive` feature
pub enum Commands {
    /// Compare two files to find the missing edge case
    #[command(author)]
    Test(TestCliArgs),

    /// Just generate the testcase
    #[command(author)]
    Generate(GeneratorCliArgs),
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

#[derive(clap::Args)]
pub struct TestCliArgs {
    /// The correct file
    #[arg(short, long, required = true, value_hint = ValueHint::FilePath)]
    pub correct_file: Option<String>,

    /// File against which you want to do test
    #[arg(short, long, required = true, value_hint = ValueHint::FilePath)]
    pub test_file: Option<String>,

    /// Clex for generating Tests
    #[arg(short, long, required = true, value_hint = ValueHint::Other)]
    pub(crate) generator: Option<String>,

    /// Max number of times of iterations
    #[arg(short, long, default_value_t = DEFAULT_ITERATIONS_COUNT, value_hint = ValueHint::Other)]
    pub(crate) iterations: usize,

    /// Whether to not stop after finding one edge case
    #[arg(short, long)]
    pub(crate) no_stop: bool,

    /// Whether or not to force recompile code even if binaries is up to date
    #[arg(short, long)]
    pub(crate) force_recompile: bool,
}

#[derive(clap::Args)]
pub struct GeneratorCliArgs {
    /// Write Clex for generating Tests
    pub(crate) generator: Option<String>,

    /// Copy testcases to clipboard
    #[arg(short, long)]
    pub(crate) clipboard: bool,
}

impl CpastCli {
    pub fn new() -> Option<Self> {
        let opt = Self::parse();
        if let Some(completions) = opt.completions {
            let mut cmd = CpastCli::command();
            eprintln!("Generating completion file for {completions:?}...");
            print_completions(completions, &mut cmd);
            None
        } else {
            Some(opt)
        }
    }
}
