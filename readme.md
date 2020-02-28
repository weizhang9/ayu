This is a command line tool to generate random password.

## How to install
### *via [crates.io](https://crates.io/crates/ayu)*
1. [install Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you haven't already done so.
> `brew install rust` on MacOS will get you rustup and cargo
2. run `cargo install ayu`

### *via downloading binary*
1. go to [releases](https://github.com/weizhang9/ayu/releases) section of the repo and download the desired binary
5. put the binary at your desired location and add the `path/to/ayu` to your $PATH in .bashrc

## Basic usage
- Run `ayu --help` to see all the information.

- Example: 
```
ayu -L=20 -C=1
```

> This command generates you a random password of a length of 20 and it contains alphameric + special charset, which is the default charset. 

> `-L` flag is required, `-C` is optional. There are 4 options to choose from. 