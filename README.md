# wordle_helper

![example workflow](https://github.com/abekoh/wordle_helper/actions/workflows/test.yml/badge.svg)

solve [Wordle](https://www.powerlanguage.co.uk/wordle/) with CLI.

![Demo](images/demo.gif)

## Requirement

Rust 1.59 or higher.

So, you have to use nightly version (at 2022.02.05).

```bash
rustup install nightly
```

## Install

```bash
cargo install --git https://github.com/abekoh/wordle_helper
```

## Usage

```
$ wordle-helper
wordle-helper 0.1.0
Solve a wordle question with CLI.

USAGE:
    wordle-helper [OPTIONS]

OPTIONS:
    -d, --dict-path <DICT_PATH>                dictionary path
    -h, --help                                 Print help information
    -m, --max-guess-count <MAX_GUESS_COUNT>    number of answer you can guess [default: 6]
    -V, --version                              Print version information
    -w, --word-length <WORD_LENGTH>            length of one word [default: 5]
```

If you use as default (words of length = 5, number of answer you can guess = 6), it's simple.  
And you can download English words from [dwyl/english-words](https://github.com/dwyl/english-words) automatically.
```bash
wordle-helper
```

You can configure some parameters.

```bash
wordle-helper -d ~/your-original-dict.txt -w 7 -m 10
```
