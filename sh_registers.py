#!/usr/bin/env python3

import os
import sys
import pathlib
import json
import re


registers_fn = 'registers.json'

home_dir = os.getenv('HOME')


permitted_pattern = r'[^a-zA-Z0-9]'

registers_path = os.path.join(home_dir, '.local/share', 'sh-registers')
pathlib.Path(registers_path).mkdir(parents=False, exist_ok=True)
registers_fn_path = os.path.join(registers_path, registers_fn)

if not os.path.exists(registers_fn_path):
    with open(registers_fn_path, 'w') as fp:
        json.dump({}, fp)

with open(registers_fn_path, 'r') as fp:
    registers = json.load(fp)

if len(sys.argv) < 2:
    raise ValueError('Too few arguments')

if sys.argv[1] in ['-r', '--remove']:
    if len(sys.argv) < 3:
        raise ValueError('Too few arguments')
    del registers[sys.argv[2]]
    with open(registers_fn_path, 'w') as fp:
        json.dump(registers, fp)
else:
    if len(sys.argv) > 2:
        if re.search(permitted_pattern, sys.argv[1]):
            raise ValueError('Invalid register name')
        command = ' '.join(sys.argv[2:])
        registers[sys.argv[1]] = command
        with open(registers_fn_path, 'w') as fp:
            json.dump(registers, fp)
    else:
        os.system(registers[sys.argv[1]])

