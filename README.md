# sh-aliases

Program to dynamically add shell aliases; shell-agnostic.


## Installation

```shell
wget https://github.com/j-morano/sh-aliases/releases/latest/download/sh-aliases
chmod +x sh-aliases
# Move to a directory in your PATH, e.g.:
mv sh-aliases $HOME/.local/bin/<desired name>
```

## Usage

```text
Usage: sh-aliases [OPTION]... [ALIAS] [COMMAND]
Add ALIAS of COMMAND.

Mandatory arguments to long options are mandatory for short options too.
  -e, --edit                 edit aliases using a text editor
  -h, --help                 display this help and exit
  -r, --remove=ALIAS         remove ALIAS
  -v, --version              display version information and exit

Exit status:
 0  if OK,
 1  if problems

Full documentation <https://github.com/j-morano/sh-aliases>\
```
