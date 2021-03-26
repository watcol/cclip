# clip
[![Crates.io](https://img.shields.io/crates/v/clip)](https://crates.io/crates/clip)
[![Downloads](https://img.shields.io/crates/d/clip)](https://crates.io/crates/clip)
[![Downloads (latest)](https://img.shields.io/crates/dv/clip)](https://crates.io/crates/clip)
[![License](https://img.shields.io/crates/l/clip)](https://github.com/watcol/clip/blob/main/LICENSE)

A simple, cross-platform CLI clipboard tool.

## Installation
```shell
$ cargo install clip
```
(Requires `cargo`)

## Usage
Get the clipboard contents:
```shell
$ clip
Hello
```

Set the clipboard contents from an argument:
```shell
$ clip Hello
```

Set the clipboard contents from stdin:
```shell
$ clip < clip.txt
```
or
```shell
$ some commands... | clip
```

## Author
- ![watcol](https://raw.githubusercontent.com/watcol/icons/main/32/normal.png) watcol

## License
clip is licensed under the MIT license. See [LICENSE](https://github.com/watcol/clip/blob/main/LICENSE) for details.
