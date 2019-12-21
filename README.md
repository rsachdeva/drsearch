## FUNCTIONALITY

Simple command 'drsearch'' to search for pattern in a file using Rust.

## DOWNLOAD for Direct Usage

Get 'drsearch' executable for mac os (x86_64-apple-darwin) in the download folder.

## INSTALLATION for Rust developers
git clone the repo 

At the root of the repo you can do:

```
cargo run <pattern> <path>
```

If you don't want to run through cargo run, then at the root of the repo you can do
```
cargo install --path .
```

You will get similar to the following messages

Finished release [optimized] target(s) in 1m 26s
Installing ~/.cargo/bin/drsearch

If you later want to uninstall it is as simple as
```
cargo uninstall drsearch
```

## USAGE
~~~~
drsearch
error: The following required arguments were not provided:
    <pattern>
    <path>

USAGE:
    drsearch <pattern> <path>
~~~~

So please provide pattern and path


## As a rust developer, You can run directly from cargo

```
cargo run "Debug" /myproject/src/main.rs
```

## Otherwise, for everyone else, assuming command 'drsearch' is in your PATH/Directory
Example

```
 drsearch "Debug" /myproject/src/main.rs
```
