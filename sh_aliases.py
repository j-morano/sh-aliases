#!/usr/bin/env python3

import os
import sys
import pathlib
import json
import re


aliases_fn = 'aliases.json'

home_dir = os.getenv('HOME')


permitted_pattern = r'[^a-zA-Z0-9]'

aliases_path = os.path.join(home_dir, '.local/share', 'sh-aliases')
pathlib.Path(aliases_path).mkdir(parents=False, exist_ok=True)
aliases_fn_path = os.path.join(aliases_path, aliases_fn)

if not os.path.exists(aliases_fn_path):
    with open(aliases_fn_path, 'w') as fp:
        json.dump({}, fp)

with open(aliases_fn_path, 'r') as fp:
    aliases = json.load(fp)

if len(sys.argv) < 2:
    print(json.dumps(aliases, indent=4))
    exit(0)

if sys.argv[1] in ['-r', '--remove']:
    if len(sys.argv) < 3:
        raise ValueError('Too few arguments')
    del aliases[sys.argv[2]]
    with open(aliases_fn_path, 'w') as fp:
        json.dump(aliases, fp)
else:
    if len(sys.argv) > 2:
        if re.search(permitted_pattern, sys.argv[1]):
            raise ValueError('Invalid alias name')
        command = ' '.join(sys.argv[2:])
        aliases[sys.argv[1]] = command
        with open(aliases_fn_path, 'w') as fp:
            json.dump(aliases, fp)
    else:
        os.system(aliases[sys.argv[1]])

