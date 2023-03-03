# Random File Generator

I created this to assist with an experiment where I was comparing [file parsing](https://github.com/thesandybridge/sandbox/tree/main/file_parse) performance
between Rust, Python, Node, and Bun.

You may find it useful.

# Installation

## Linux

The downloaded file will be located in `~/.local/bin/`

```bash
curl -s https://tinyurl.com/thesandybridge-gf | bash
```

# Basic Usage

By default, if no line number is defined, 1000 lines of 8 character Alphanumeric strings are generated.
I may add some verbose output in the future.

```bash
gfc list.txt

gfc --lines 1000 list.txt

gfc --lines 1000 --characters 8 list.txt
```
