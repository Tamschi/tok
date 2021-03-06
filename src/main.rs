use {
    structopt::StructOpt,
    tok::{self, Entry, Span},
};

/// Simple-ish time tracking from the command line.
///
/// You can use a text editor to view/edit the tracking file. The parser should be lenient.
#[derive(Debug, StructOpt)]
#[structopt(author)]
struct Options {
    /// Disables searching parent directory chain for time tracking files
    #[structopt(short = "W", long)]
    no_walk: bool,
    #[structopt(subcommand)]
    command: Option<Command>,
    /// Either adds tags to the tracked time (start) or filters by them (, stats, stop)
    /// Separate with ,
    /// Prefix tags with ! for negative filters
    #[structopt(short, long, verbatim_doc_comment)]
    tags: Option<String>,
}

#[derive(Debug, StructOpt, PartialEq)]
enum Command {
    /// Print current status information.
    #[structopt(name = "(none)")]
    None,
    /// Create a time tracking file (.tok-tracker) in the current working directory.
    Init,
    /// Start tracking time.
    Start {
        /// Adds comments to the new time range.
        #[structopt(short, long)]
        comments: Vec<String>,
    },
    /// Stop tracking time.
    Stop {
        /// Adds comments to each finished time range.
        #[structopt(short, long)]
        comments: Vec<String>,
    },
    /// Displays various tallies.
    Stats,
    /// Rewrites the time tracking file with canonical formatting.
    Touch,
}

#[allow(unreachable_code)]
fn main() {
    let options = Options::from_args();

    let command = options.command.unwrap_or(Command::None);

    if command == Command::Init {
        drop(tok::init().unwrap());
        println!("Created tracking file in current directory. Have fun!");
        return;
    }

    let file_path =
        tok::find_tracking_file(!options.no_walk).expect("Failed to find tracking file.");

    let tags: Vec<String> = options
        .tags
        .into_iter()
        .flat_map(|t| t.split(',').map(|t| t.to_string()).collect::<Vec<_>>())
        .collect();

    let data: Vec<tok::Entry> =
        tok::load(&file_path).expect("Could not load tracking file content");

    use Command::*;
    let data: Vec<tok::Entry> = match command {
        None => {
            if data
                .into_iter()
                .filter(|entry| match entry {
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
                        true
                    }
                    Entry {
                        span: Span::Closed { .. },
                        ..
                    } => false,
                })
                .count()
                == 0
            {
                println!("No open time spans.")
            }
            return;
        }

        Init => unreachable!(),

        Start { comments } => {
            assert!(
                !tags.iter().any(|t| t.starts_with('!')),
                "Found tag starting with !"
            );
            let mut data: Vec<Entry> = data.into_iter().collect();
            data.push(Entry {
                span: Span::Active {
                    start: time::OffsetDateTime::try_now_local()
                        .expect("Could not determine time zone offset"),
                },
                tags,
                comments,
            });
            data
        }

        Stop { comments } => {
            let mut stopped_count = 0;
            let data = data
                .into_iter()
                .map(|entry| {
                    let tag_match = tags.iter().all(|arg_tag| {
                        if arg_tag.starts_with('!') {
                            assert!(!(&arg_tag[1..]).contains('!'), "Tags must not contain !");
                            entry
                                .tags
                                .iter()
                                .all(|entry_tag| &arg_tag[1..] != entry_tag)
                        } else {
                            assert!(!arg_tag.contains('!'), "Tags must not contain !");
                            entry.tags.iter().any(|entry_tag| arg_tag == entry_tag)
                        }
                    });
                    if tag_match {
                        match entry.span {
                            Span::Active { start } => {
                                stopped_count += 1;
                                return Entry {
                                    span: Span::Closed {
                                        start,
                                        end: time::OffsetDateTime::try_now_local()
                                            .expect("Could not determine time zone offset"),
                                    },
                                    comments: entry
                                        .comments
                                        .into_iter()
                                        .chain(comments.iter().cloned())
                                        .collect(),
                                    ..entry
                                };
                            }
                            Span::Closed { .. } => (),
                        }
                    }
                    entry
                })
                .collect();
            println!("Stopped {}.", stopped_count);
            data
        }

        Stats => {
            //TODO?: This part is very rudimentary.
            //TODO: Filter by tags.
            data.into_iter().for_each(|entry| {
                println!(
                    "{}\t{}\t({})\t{}",
                    entry.span.start(),
                    entry
                        .span
                        .duration()
                        .map_or("".to_owned(), |duration| format!(
                            "{}h{}min",
                            duration.whole_hours(),
                            duration.whole_minutes() % 60
                        )),
                    entry.tags.join(","),
                    std::iter::repeat("#")
                        .zip(entry.comments.iter())
                        .flat_map(|(a, b)| a.chars().chain(b.chars()))
                        .collect::<String>()
                )
            });
            return;
        }

        Touch => data,
    };

    tok::update(&file_path, data.as_slice()).expect("Could not update tracking file");
}
