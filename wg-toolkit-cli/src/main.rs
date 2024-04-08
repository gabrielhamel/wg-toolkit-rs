//! The CLI for wg-toolkit library.

use std::io::{self, IsTerminal};
use std::process::ExitCode;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

mod pxml;
mod res;


/// Global options for the command line interface.
#[derive(Debug)]
pub struct CliOptions {
    /// Human readable mode enabled.
    pub human: bool,
}

/// Command line utility for interacting with codecs distributed by Wargaming.net studio.
/// 
/// This command line tries to provide UNIX-oriented commands that can be piped together
/// to make more complex operations.
#[derive(Debug, Parser)]
#[command(version, author, disable_help_subcommand = true, max_term_width = 100)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
    /// Optionally force human readable mode or not.
    /// 
    /// This is automatically enabled if stdout is a terminal, so it's automatically
    /// disabled when piping in your shell to a file or another program. This make 
    /// interoperability with UNIX like programs easier. Human readable output cannot
    /// be easily parsed!
    #[arg(short = 'H', long)]
    pub human: Option<bool>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(name = "pxml")]
    PackedXml(PackedXmlArgs),
    Res(ResArgs)
}

/// Packed XML read and write utilities.
/// 
/// This is a format that is commonly used in game resources, it provides a kind of 
/// serialization for a XML, packed XML files use the same extension as regular XML 
/// (.xml), a packed XML file can be replaced by a clear XML file and will work the
/// same.
#[derive(Debug, Args)]
pub struct PackedXmlArgs {
    /// If specified, the packed XML is read from a file instead of stdin (fd 0).
    /// 
    /// This is essentially the same as piping 'cat' of the packed XML file into it. 
    #[arg(short, long)]
    pub file: Option<PathBuf>,
    /// Enable XML output style.
    /// 
    /// This can be used if you want to replace a packed XML file with a clear one, 
    /// this flag will correctly format the packed XML as a regular clear XML file
    /// that can be read by the game engine.
    #[arg(short, long)]
    pub xml: bool,
    /// If needed, the packed XML can be modified before outputting it.
    /// 
    /// The filter is basically a sequence of statements, with an expression at the end
    /// that dictates what value to output. Each statement must end with a semicolon ';'.
    /// 
    /// An expression is something that returns a packed XML value: Element, 
    /// String ("hello world"), Integer (64-bit signed), Boolean (true, false),
    /// Float (32-bit IEEE 754), Vec3, Affine3.
    pub filter: Option<String>,
}

/// Game resources virtual filesystem access (readonly).
/// 
/// The game resources are split in many directories under the game's resources (res/)
/// directory, most of resources are actually stored inside huge package files (.pkg).
/// This command uses efficient indexing on these packages to efficiently fetch and
/// interact with the files, this works with not-packaged files and packaged files at
/// the same time.
#[derive(Debug, Args)]
pub struct ResArgs {
    /// Path to the game's resource (res/) directory.
    pub dir: PathBuf,
    #[command(subcommand)]
    pub cmd: ResCommand
}

#[derive(Debug, Subcommand)]
pub enum ResCommand {
    Read(ResReadArgs),
    #[command(name = "ls")]
    List(ResListArgs),
    #[command(name = "cp")]
    Copy(ResCopyArgs),
}

/// Read a file and write its content on the standard output.
/// 
/// Like 'ls', this command may take some time to complete depending on where the file is
/// located in packages, this command return as soon as possible so you may be lucky if
/// the file is located in first opened packages.
#[derive(Debug, Args)]
pub struct ResReadArgs {
    /// Path to the file to read, no leading separator!
    pub path: String,
}

/// List directory contents with optional recursion.
/// 
/// Note that this function may take a really long time to proceed, because all packages
/// needs to be opened to ensures that the given directory is present or not. This should
/// be faster on subsequent calls because of your operating system filesystem cache.
#[derive(Debug, Args)]
pub struct ResListArgs {
    /// Path to the directory to list, no leading separator (empty to list root)!
    #[arg(default_value = "")]
    pub path: String,
    /// Enable recursion listing of directories.
    /// 
    /// By default this will recurse indefinitely, but you can provide a limit to the 
    /// recursion, for example '1' will show children of all root directories.
    #[arg(short, long)]
    pub recurse: Option<Option<u16>>,
}

/// Copy files and directories from resources.
#[derive(Debug, Args)]
pub struct ResCopyArgs {
    /// Source path of the file or directory to copy from resources.
    /// 
    /// Trailing separator '/' for directories is not necessary, where the file or 
    /// directory is copied is controlled by the destination path.
    #[arg(required = true)]
    pub source: Vec<String>,
    /// Destination directory, in your native filesystem.
    /// 
    /// The destination directory must exists. In general, this will error out if a file 
    /// is copied onto an existing directory, or if a directory is copied onto a existing 
    /// file, or for many other I/O errors.
    pub dest: PathBuf,
}

/// Type alias for a result that simply returns a string on error, this will be output
/// on stderr and process returns a failed exit code. This allows easier error handling
/// by just mapping the error type to an explanatory text.
pub type CliResult<T> = Result<T, String>;

/// Entrypoint.
fn main() -> ExitCode {

    let args = Cli::parse();
    let opts = CliOptions {
        human: args.human.unwrap_or_else(|| io::stdout().is_terminal()),
    };

    let res = match args.cmd {
        Command::PackedXml(args) => pxml::cmd_pxml0(args),
        Command::Res(args) => res::cmd_res(opts, args),
    };

    if let Err(message) = res {
        eprintln!("{message}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
    
}
