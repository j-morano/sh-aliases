#!/usr/bin/env python3

import os
import sys
import json
import re


# Specify the path to the JSON file where you want to store the aliases
aliases_fn = ''


help = """\
Usage: sh_aliases.py [OPTION]... [ALIAS] [COMMAND]
Add ALIAS of COMMAND.

Mandatory arguments to long options are mandatory for short options too.
  -r, --remove=ALIAS         remove ALIAS
  -h, --help                 display this help and exit

Exit status:
 0  if OK,
 1  if problems

Full documentation <https://github.com/sonarom/sh-aliases>\
"""


permitted_pattern = r'[^a-zA-Z0-9]'


aliases = None
if not os.path.exists(aliases_fn):
    aliases = {}
else:
    with open(aliases_fn, 'r') as fp:
        aliases = json.load(fp)

if len(sys.argv) < 2:
    print(json.dumps(aliases, indent=4))
else:
    if sys.argv[1] in ['-h', '--help']:
        print(help)
    elif sys.argv[1] in ['-r', '--remove']:
        if len(sys.argv) < 3:
            raise ValueError('Too few arguments')
        del aliases[sys.argv[2]]
        with open(aliases_fn, 'w') as fp:
            json.dump(aliases, fp)
    else:
        if len(sys.argv) > 2:
            if re.search(permitted_pattern, sys.argv[1]):
                raise ValueError('Invalid alias name')
            command = ' '.join(sys.argv[2:])
            aliases[sys.argv[1]] = command
            with open(aliases_fn, 'w') as fp:
                json.dump(aliases, fp)
        else:
            os.system(aliases[sys.argv[1]])
