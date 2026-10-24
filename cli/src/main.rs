mod call;
mod cargo;
mod clean;
mod compile;
mod dev;
mod library;
mod new;
mod paths;
mod project;
mod restart;
mod run;
mod target;
mod watch;

use clap::{Args, Parser, Subcommand};
use std::process::ExitCode;

#[derive(Parser, Debug, Clone)]
#[command(name = "amane")]
struct CommandArgs {
    /// TODO
    #[clap(subcommand)]
    subcommand: Subcommands,
}

#[derive(Debug, Subcommand, Clone)]
enum Subcommands {
    /// TODO
    New {
        #[command(flatten)]
        args: NewArgs,
    },
    /// TODO
    Startup {
        #[command(flatten)]
        args: StartupArgs,
    },
    /// TODO
    Target {
        #[command(flatten)]
        args: TargetArgs,
    },
    /// TODO
    Compile {
        #[command(flatten)]
        args: CompileArgs,
    },
    /// TODO
    Run,
    /// TODO
    Restart,
    /// TODO
    Dev,
    /// TODO
    Clean {
        #[command(flatten)]
        args: CleanArgs,
    },
    /// TODO
    Ipc {
        #[command(flatten)]
        args: IpcArgs,
    },
}

#[derive(Args, Debug, Clone)]
struct NewArgs {
    /// TODO
    #[arg(long, default_value_t = false)]
    example: bool,

    /// TODO
    folder: Option<String>,
}

#[derive(Args, Debug, Clone)]
struct StartupArgs {
    /// TODO
    #[arg(long, default_value_t = false)]
    example: bool,
}

#[derive(Args, Debug, Clone)]
struct TargetArgs {
    /// TODO
    folder: Option<String>,
}

#[derive(Args, Debug, Clone)]
struct IpcArgs {
    /// TODO
    #[arg(value_parser =  clap::builder::PossibleValuesParser::new(["call"]))]
    _action: String,

    /// TODO
    name: String,

    /// TODO
    #[arg(trailing_var_arg = true)]
    args: Vec<String>,
}

#[derive(Args, Debug, Clone)]
struct CleanArgs {
    /// TODO
    #[arg(long, default_value_t = false)]
    yes: bool,
}

#[derive(Args, Debug, Clone)]
struct CompileArgs {
    /// TODO
    #[arg(long, default_value_t = false)]
    restart: bool,
}

fn main() -> ExitCode {
    let args = CommandArgs::parse();
    match args.subcommand {
        Subcommands::New { args } => new::run(args.folder.as_deref(), args.example),
        Subcommands::Startup { args } => startup(args.example),
        Subcommands::Target { args } => target::run(args.folder.as_deref()),
        Subcommands::Compile { args } => compile::run(args.restart),
        Subcommands::Run => run::run(),
        Subcommands::Restart => restart::run(),
        Subcommands::Dev => dev::run(),
        Subcommands::Clean { args } => clean::run(args.yes),
        Subcommands::Ipc { args } => call::run(&args.name, &args.args),
    }
}

// the old name for `amane new`, kept for a while so existing setups don't break
fn startup(example: bool) -> ExitCode {
    eprintln!(
        "warning: `amane startup` is deprecated and will be removed, use `amane new` instead"
    );

    new::run(None, example)
}
