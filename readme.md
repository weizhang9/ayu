This is a command line tool version of random password generator. I used this project to learn Rust so there are a lot improvements can be done, until then I will keep it in this repo instead of publishing for public usage. Feel free to clone it locally to build a binary to run in your command line if you think this would be useful.

## How to use
1. clone this repo
2. in the root of this repo, run `cargo build --release`
3. now you will see a `target` directory generated
4. under `./target/release` you can see the binary `ayu` which is this tool
5. put the binary at your desired location on your Mac (sorry it only supports MacOS at the moment)
6. go to the directory where you store `ayu` and run `./ayu --help`

### Basic usage
- you can run `./ayu -L=20 -C=1`
This command generates your a random password of a length of 20 and it contains alphameric + special charset, which is the default charset. You could leave the `-C` flag if you want the default charset, otherwise there are 4 options to choose from. You can run `./ayu --help` to see all the options.