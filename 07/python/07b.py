from __future__ import annotations

import sys
from functools import reduce
from dataclasses import dataclass, field
from pathlib import Path

@dataclass
class File:
    name: str
    size: int

    def tree(self, indent: int = 0) -> str:
        return indent * ' ' + f'- {self.name} (file, {self.size})\n'

@dataclass
class Dir:
    name: str
    files: list[File] = field(default_factory=list)
    dirs: dict[str, Dir] = field(default_factory=dict)
    parent: Dir | None = None

    def tree(self, indent: int = 0) -> str:
        files_and_dirs = [*self.files, *self.dirs.values()]
        files_and_dirs = sorted(files_and_dirs, key=lambda x: x.name)
        s = indent * ' ' + f'- {self.name}\n'
        for x in files_and_dirs:
            s += x.tree(indent=indent+2)
        return s
    
    @property
    def size(self) -> int:
        files_and_dirs = [*self.files, *self.dirs.values()]
        sizes = map(lambda x: x.size, files_and_dirs)
        return sum(sizes)
    
    def child_dirs_and_self(self) -> list[Dir]:
        result = [self]
        for d in self.dirs.values():
            result.extend(d.child_dirs_and_self())
        return result



filename = sys.argv[1]
lines = Path(filename).read_text().strip().split('\n')

i = 0
current_dir = None
root_dir = None
logging = False

while i < len(lines):
    # Assume we're starting with a command.
    line = lines[i].strip()
    print("-----")
    print("line:", line)
    if not line.startswith('$ '):
        print('oh no....')
        break

    command = line.split()[1]

    if command == 'cd':
        print('found cd command')
        newdir = line.split()[2]
        if current_dir is None:
            print(f'Creating initial dir {newdir}')
            current_dir = Dir(newdir)
            root_dir = current_dir
        else:
            if newdir == '..':
                print(f'back down a level, going from {current_dir.name} to {current_dir.parent.name}')
                current_dir = current_dir.parent
            else:
                print(f'entering {newdir} from {current_dir.name}')
                current_dir = current_dir.dirs[newdir]
        i += 1

    elif command == 'ls':
        i += 1
        # Keep reading lines until we find another command line:
        while i < len(lines) and not lines[i].startswith('$'):
            line = lines[i]
            parts = line.strip().split()
            if parts[0] == 'dir':
                newdir_name = parts[1]
                newdir = Dir(newdir_name, parent=current_dir)
                if newdir_name in current_dir.dirs:
                    raise RuntimeError("This is just weird, we've seen this dir before")
                print(f'Creating new dir "{newdir_name}" in "{current_dir.name}"')
                current_dir.dirs[newdir_name] = newdir
                i += 1
            else:
                size, name = parts
                newfile = File(name, int(size))
                print(f'Creating new file "{newfile}" in "{current_dir.name}"')
                current_dir.files.append(newfile)
                i += 1

    else:
        raise RuntimeError('wtf')

total_size = root_dir.size
unused = 70000000 - total_size
deficit = 30000000 - unused

viable_deletes = [d for d in root_dir.child_dirs_and_self() if d.size >= deficit]
best_option = sorted(viable_deletes, key=lambda x: x.size)[0]

print(best_option.name)
print(best_option.size)