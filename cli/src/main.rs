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

/// Blazingly fast library for building Wayland dekstop shells.
///
///
#[derive(Parser, Debug, Clone)]
#[command(name = "amane", version)]
struct CommandArgs {
    #[clap(subcommand)]
    subcommand: Subcommands,
}

#[derive(Debug, Subcommand, Clone)]
enum Subcommands {
    /// Create a new shell configuration
    New {
        #[command(flatten)]
        args: NewArgs,
    },
    /// Deprecated, use amane new instead.
    #[command(hide = true)]
    Startup {
        #[command(flatten)]
        args: StartupArgs,
    },
    /// Show or set the active shell directory
    Target {
        #[command(flatten)]
        args: TargetArgs,
    },
    /// Build the optimized shell binary
    Compile {
        #[command(flatten)]
        args: CompileArgs,
    },
    /// Start the compiled shell
    Run,
    /// Stop the running shell and start the compiled one
    Restart,
    /// Rebuild and restart the shell on every config save
    Dev,
    /// Clear the local build cache, keeps the compiled shell
    Clean {
        #[command(flatten)]
        args: CleanArgs,
    },
    /// Call a handler in the running shell.
    Ipc {
        #[command(subcommand)]
        action: IpcSubcommand,
    },
}

#[derive(Args, Debug, Clone)]
struct NewArgs {
    /// Provide an example configuration template
    #[arg(long, default_value_t = false)]
    example: bool,

    /// Target installation directory [default: ~/.config/amane]
    #[arg(value_name = "FOLDER")]
    folder: Option<String>,
}

#[derive(Args, Debug, Clone)]
struct StartupArgs {
    #[arg(long, default_value_t = false)]
    example: bool,
}

#[derive(Args, Debug, Clone)]
struct TargetArgs {
    /// Changes where amane looks for your shell
    folder: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
enum IpcSubcommand {
    /// <NAME> [ARGS]...  Call a handler with the given name.
    Call {
        /// Name of the target hook handle
        name: String,

        /// Dynamic list of args to be sent to the hook
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
}

#[derive(Args, Debug, Clone)]
struct CleanArgs {
    /// Bypass the verification prompt and purge immediately
    #[arg(long, default_value_t = false)]
    yes: bool,
}

#[derive(Args, Debug, Clone)]
struct CompileArgs {
    /// Trigger an instant hot-reload restart upon compiling successfully
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
        Subcommands::Ipc { action } => match action {
            IpcSubcommand::Call { name, args } => call::run(&name, &args),
        },
    }
}

// the old name for `amane new`, kept for a while so existing setups don't break
fn startup(example: bool) -> ExitCode {
    eprintln!(
        "warning: `amane startup` is deprecated and will be removed, use `amane new` instead"
    );

    new::run(None, example)
}
