# Tok

Simple-ish time tracking from the command line.

## Installation

```cmd
cargo install --git https://github.com/Tamschi/tok --branch master
```

or use a binary from <https://github.com/Tamschi/tok/releases>.

## Help

```txt
>tok help
tok 0.0.0-dev
Tamme Schichler <tamme@schichler.dev>
Simple-ish time tracking from the command line.

You can use a text editor to view/edit the tracking file. The parser should be lenient.

USAGE:
    tok [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -W, --no-walk    Disables searching parent directory chain for time tracking files
    -V, --version    Prints version information

OPTIONS:
    -t, --tags <tags>    Either adds tags to the tracked time (start) or filters by them (, stats, stop)
                         Separate with ,
                         Prefix tags with ! for negative filters

SUBCOMMANDS:
    (none)    Print current status information
    help      Prints this message or the help of the given subcommand(s)
    init      Create a time tracking file (.tok-tracker) in the current working directory
    start     Start tracking time
    stats     Displays various tallies
    stop      Stop tracking time
    touch     Rewrites the time tracking file with canonical formatting

>tok help start
tok-start 0.0.0-dev
Start tracking time

USAGE:
    tok start [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --comments <comments>...    Adds comments to the new time range

>tok help stop
tok-stop 0.0.0-dev
Stop tracking time

USAGE:
    tok stop [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --comments <comments>...    Adds comments to each finished time range
```
