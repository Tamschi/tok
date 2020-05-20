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
tok 1.0.1
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
tok-start 1.0.1
Start tracking time

USAGE:
    tok start [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --comments <comments>...    Adds comments to the new time range

>tok help stop
tok-stop 1.0.1
Stop tracking time

USAGE:
    tok stop [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --comments <comments>...    Adds comments to each finished time range
```

## Example

```cmd
C:\Users\Tamme>tok init
Created tracking file in current directory. Have fun!

C:\Users\Tamme>tok -t work,home start

C:\Users\Tamme>tok stop -c "That's it for today!"
Stopped 1.

C:\Users\Tamme>tok stats
2020-05-03 15:11:48 +2  0h4min  (work,home)     #That's it for today!
```

## Versioning

Tok uses semantic versioning, with the following exceptions:

- Program output is not versioned (except return codes, which are).
- Changes to the `.tok-tracker` file format are only considered breaking if previous versions of Tok fail to read them.

## Library

Tok's tracker file layer is a library crate you can use to write compatible tools:

```toml
[dependencies]
tok = { git = "https://github.com/Tamschi/tok", branch = "1" }
```

(However, I haven't sorted out licensing yet. Consider waiting until I do.)

## License

Until I find an appropriate license for this project, it is licensed under [CC BY-ND 4.0](https://creativecommons.org/licenses/by-nd/4.0/).  
This should for now give you enough room to use the program, but doesn't work for accepting contributions.
