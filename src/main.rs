use {atty::Stream, structopt::StructOpt, tok};

/// Simple-ish time tracking from the command line.
///
/// You can use a text editor to view/edit the tracking file. The parser should be lenient.
#[derive(Debug, StructOpt)]
#[structopt(author)]
struct Options {
    /// Enables colour.
    #[structopt(short, long)]
    color: bool,
    /// Disables colour. Overrides --color.
    #[structopt(short = "C", long)]
    no_color: bool,
    /// Disables searching parent directory chain for time tracking files.
    #[structopt(short = "W", long)]
    no_walk: bool,
    #[structopt(subcommand)]
    command: Option<Command>,
    /// Either adds tags to the edited time (start, stop) or filters by them (, stats).
    #[structopt(short, long)]
    tags: Vec<String>,
}

#[derive(Debug, StructOpt, PartialEq)]
enum Command {
    /// Print current status information.
    #[structopt(name = "")]
    None,
    /// Create a time tracking file (.tok-tracker) in the current working directory.
    Init,
    /// Start tracking time.
    Start {
        /// Adds a comment to the new time range.
        #[structopt(short, long)]
        comment: Option<String>,
    },
    /// Stop tracking time.
    Stop {
        /// Adds a comment to the finished time range.
        #[structopt(short, long)]
        comment: Option<String>,
    },
    /// Displays various tallies.
    Stats,
}

fn main() {
    let options = Options::from_args();

    let _color = if options.no_color {
        false
    } else if options.color {
        true
    } else {
        atty::is(Stream::Stdout)
    };

    let command = options.command.unwrap_or(Command::None);

    if command == Command::Init {
        drop(tok::init().unwrap());
        println!("Created tracking file in current directory. Have fun!");
        return;
    }
}
