# sh-aliases

Program to dynamically add shell aliases; shell-agnostic.


## Installation

Clone repository
```shell
git clone git@github.com:sonarom/sh-aliases.git
```
Then, it is necessary to modify, in the program, the path to the file where the aliases will be stored.
```shell
cd sh-aliases
vi sh_aliases.py
# -> change 'aliases_fn'
```
 
Create link to the program in a PATH directory; e.g.:
```shell
ln -s /path/to/sh-aliases/sh_aliases.py ~/.local/bin/a
```

## Usage

```text
Usage: sh_aliases.py [OPTION]... [ALIAS] [COMMAND]
Add ALIAS of COMMAND.

Mandatory arguments to long options are mandatory for short options too.
  -r, --remove=ALIAS         remove ALIAS
  -e, --edit                 edit aliases using a text editor
  -h, --help                 display this help and exit

Exit status:
 0  if OK,
 1  if problems

Full documentation <https://github.com/sonarom/sh-aliases>\
```

If you created a link to the script, then you can run the program with the correspondig name, e.g. `a`.
