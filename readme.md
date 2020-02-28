This is a command line tool version of random password generator.

## How to use
1. go to {{releases}} section of the repo and download the desired binary
5. put the binary at your desired location and add the `path/to/ayu` to your $PATH in .bashrc
6. run `ayu --help` in your command line to see options of the tool

### Basic usage
- you can run `ayu -L=20 -C=1`  

This command generates you a random password of a length of 20 and it contains alphameric + special charset, which is the default charset. You could leave the `-C` flag if you want the default charset, otherwise there are 4 options to choose from. You can run `./ayu --help` to see all the options.
