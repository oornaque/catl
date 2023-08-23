# catl
Concatenate multiple lines into a single one

## Usage
```
Usage: catl.exe [OPTIONS] [string]

Arguments:
  [string]  Input from the cli arguments. Leave it blank and do not set a file to read from stdin

Options:
  -f, --file <file>    File to read the input from
  -n, --ommit_newline  Ommit the newline when printing the output. Useful when redirecting the output to a file
  -h, --help           Print help
```
### Examples
```
$ catl
qwe
asd
zxc
qweasdzxc
```
