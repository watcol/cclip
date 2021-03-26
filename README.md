# clip
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
