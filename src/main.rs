use {
    atty::Stream,
    std::fs::File,
    structopt::StructOpt,
    tok::{self, Entry, Span},
};

/// Simple-ish time tracking from the command line.
///
/// You can use a text editor to view/edit the tracking file. The parser should be lenient.
#[derive(Debug, StructOpt)]
#[structopt(author)]
struct Options {
    /// Enables colour
    #[structopt(short, long)]
    color: bool,
    /// Disables colour
    /// Overrides --color
    #[structopt(short = "C", long, verbatim_doc_comment)]
    no_color: bool,
    /// Disables searching parent directory chain for time tracking files
    #[structopt(short = "W", long)]
    no_walk: bool,
    #[structopt(subcommand)]
    command: Option<Command>,
    /// Either adds tags to the tracked time (start) or filters by them (, stats, stop)
    /// Separate with ,
    /// Prefix tags with ! for negative filters
    #[structopt(short, long, verbatim_doc_comment)]
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
    /// Rewrites the time tracking file with canonical formatting.
    Touch,
}

#[allow(unreachable_code)]
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

    let file_path =
        tok::find_tracking_file(!options.no_walk).expect("Failed to find tracking file.");
    let old_file = File::open(file_path).expect("Could not open tracking file");

    let tags: Vec<String> = options
        .tags
        .into_iter()
        .flat_map(|t| t.split(',').map(|t| t.to_string()).collect::<Vec<_>>())
        .collect();

    let data: Vec<tok::Entry> = vec![]; //TODO

    use Command::*;
    match command {
        None => {
            if data.into_iter().all(|entry| match entry {
                Entry {
                    span: Span::Active { start },
                    tags,
                    comments,
                } => {
                    println!(
                        "Tracking{} since {} ({} comments)",
                        if tags.is_empty() {
                            "".to_owned()
                        } else {
                            format!(" ({})", tags.join(","))
                        },
                        start,
                        comments.len()
                    );
                    false
                }
                Entry {
                    span: Span::Closed { .. },
                    ..
                } => true,
            }) {
                println!("No open time spans.")
            }
            return;
        }
        Init => unreachable!(),
        Start { comment } => todo!(),
        Stop { comment } => todo!(),
        Stats => todo!(),
        Touch => todo!(),
    }
}
