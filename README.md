# Welcome to peppergrep!
This simple grep utility was made by following the 12th chapter of the _Rust Book_ (formally known as _The Rust Programming Language_). Some small changes have been made.

## Installation
You can install this simple utility via [`cargo install`](https://doc.rust-lang.org/cargo/commands/cargo-install.html):

```shell
cargo install peppergrep 
```
If the installation completes successfully, you're all set to use peppergrep!

## Usage
To use this utility, just run:

```shell
peppergrep <query> <source_file> [--ignore-case / --attend-case]
```
where `query` is the string you're looking for, `source_file` is the path of the file you're searching the query in. `--ignore-case` and `--attend-case` are optional arguments that you can specify to force case insensitivity or sensitivity. By default, all searches are case-sensitive. You can override this default behavior by creating an environmental variable called `IGNORE_CASE`.

On linux, it would be:

```shell
IGNORE_CASE = 1
```

On windows (PowerShell):

```pwsh
$env:IGNORE_CASE = 1;
```

## Uninstall
To uninstall this utility, just run:

```shell
cargo uninstall peppergrep
```

