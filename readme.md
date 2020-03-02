This is a command line tool to generate random password.

## How to install
### *via [crates.io](https://crates.io/crates/ayu)* (supports 32-bit & 64-bit Windows, Linux and MacOS)
1. [install Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you haven't already done so.
> `brew install rust` on MacOS will get you rustup and cargo
2. run `cargo install ayu`

### *via downloading binary* (supports 64-bit Linux and MacOS and 32-bit & 64-bit Windows)
1. go to [releases](https://github.com/weizhang9/ayu/releases) section of the repo and download the desired binary
5. put the binary at your desired location and add the `path/to/ayu` to your `$PATH` variable in your .bashrc file

## Basic usage
- Run `ayu -h` to see all the information.
```
USAGE:
    ayu [FLAGS] [OPTIONS]

FLAGS:
    -N, --lowercase    Return password in LOWERCASE, invalid if used with UPPERCASE flag -U
    -U, --uppercase    Return password in UPPERCASE, invalid if used with LOWERCASE flag -N
    -h, --help         Prints help information
    -V, --version      Prints version information

OPTIONS:
    -C, --char <CHARSET>    [OPTIONAL] desired password CHARSET
                            Options include:
                            1) Alphanumeric + Special characters (default)
                            2) Alphanumeric
                            3) Alphabetic
                            4) Numberic

    -L, --len <LENGTH>      [OPTIONAL] desired password LENGTH (default: 18)
```

- Example: 
```
ayu
```

> This command generates a random password of a length of 18 and it contains alphameric + special charset, which is the default charset. 

```
ayu -L=20 -C=2 -U
```

> This command generates a random password of a length of 20 and it contains uppercase alphameric.

## Maintenance and Contribution
This tool is passively maintained. If you encountered an issue or have any suggestions, please submit [here](https://github.com/weizhang9/ayu/issues). Contributions are always welcome. :)