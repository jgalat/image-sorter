# image-sorter

![Demo](.github/screenshot.jpg)

A terminal user interface for sorting images. It requires w3m to render the images.

Based on this [thread](https://boards.4channel.org/g/thread/78507445).

## Installation

The binary executable is `image-sorter`.

### Repository

Requires `cargo` to be installed.

Clone or download this repository and run the following inside the project

```bash
cargo install --path .
```

## Usage

Once installed, run the following to print the help message

```bash
image-sorte --help
```

Here is an example usage

```bash 
image-sorter -b w ~/4/wg -b g ~/4/g -o run.sh -- image.jpg ~/Downloads/
```

Running the command above will configure the program like this
- bind `w` to the path `~/4/wg`
- bind `g` to the path `~/4/g`
- set `run.sh` as the output script of the program
- the software will list `image.jpg` and all the images inside `~/Downloads/` so they can be sorted
