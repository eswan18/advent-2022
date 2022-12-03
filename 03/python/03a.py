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
    compartments: tuple[str, str]

    def common_element(self):
        half0, half1 = self.compartments
        common = set(half0).intersection(set(half1))
        if len(common) != 1:
            raise RuntimeError
        return common.pop()
    
    def value(self):
        common = self.common_element()
        return priority(common)

    @classmethod
    def from_line(cls, line: str) -> 'Rucksack':
        halfpoint = int(len(line) / 2)
        halves = line[:halfpoint], line[halfpoint:]
        return cls(halves)
        

filename = sys.argv[1]
lines = Path(filename).read_text().strip().split('\n')

rucksacks = [Rucksack.from_line(line) for line in lines]
values = [r.value() for r in rucksacks]
print(values)
result = sum(values)
print(result)