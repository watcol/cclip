# Cross Clip
[![Crates.io](https://img.shields.io/crates/v/cclip)](https://crates.io/crates/cclip)
[![Downloads](https://img.shields.io/crates/d/cclip)](https://crates.io/crates/cclip)
[![Downloads (latest)](https://img.shields.io/crates/dv/cclip)](https://crates.io/crates/cclip)
[![License](https://img.shields.io/crates/l/cclip)](https://github.com/watcol/clip/blob/main/LICENSE)

A simple, cross-platform CLI clipboard tool.

## Installation
```shell
$ cargo install cclip
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
Cross Clip is licensed under the MIT license. See [LICENSE](https://github.com/watcol/clip/blob/main/LICENSE) for details.
