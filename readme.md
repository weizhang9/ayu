This is a command line tool version of random password generator.

## How to use
### *via [crates.io](https://crates.io/crates/ayu)*
1. [install `Cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you haven't already done so.
> `brew install rust` on MacOS will get you all the rustup and cargo
2. run `cargo install ayu`
3. run `ayu --help` to see available flags for the tool

### *via downloading binary*
1. go to [releases](https://github.com/weizhang9/ayu/releases) section of the repo and download the desired binary
5. put the binary at your desired location and add the `path/to/ayu` to your $PATH in .bashrc
6. run `ayu --help` to see available flags for the tool

## Basic usage
- run `ayu --help` to see all the options.

- Example: 
```
ayu -L=20 -C=1 

This command generates you a random password of a length of 20 and it contains alphameric + special charset, which is the default charset. 

You could leave the `-C` flag if you want the default charset, otherwise there are 4 options to choose from. 
```

