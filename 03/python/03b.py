import sys
from dataclasses import dataclass
from pathlib import Path

def priority(letter: str) -> int:
    if 'a' <= letter <= 'z':
        return ord(letter) - (ord('a')) + 1
    if 'A' <= letter <= 'Z':
        return ord(letter) - (ord('A')) + 27

@dataclass
class Rucksack:
    items: set[str]

    @classmethod
    def from_line(cls, line: str) -> 'Rucksack':
        return cls(set(line))
    
@dataclass
class Group:
    sacks: tuple[Rucksack, Rucksack, Rucksack]

    def common_item(self):
        common = self.sacks[0].items
        common = common.intersection(self.sacks[1].items)
        common = common.intersection(self.sacks[2].items)
        if len(common) != 1:
            raise RuntimeError
        return common.pop()
    
    def value(self):
        return priority(self.common_item())

filename = sys.argv[1]
lines = Path(filename).read_text().strip().split('\n')

groups = []
n_groups = int(len(lines) / 3)
for i in range(n_groups):
    start_index = i * 3
    end_index = start_index + 3
    sacks = tuple(Rucksack.from_line(line) for line in lines[start_index:end_index])
    groups.append(Group(sacks))

values = [g.value() for g in groups]
print(values)
result = sum(values)
print(result)