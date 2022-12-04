import sys
from dataclasses import dataclass
from pathlib import Path

@dataclass
class Range:
    start: int
    stop: int

    @classmethod
    def from_string(cls, string: str) -> 'Range':
        start, stop = string.split('-')
        start = int(start)
        stop = int(stop)
        return cls(start, stop)
    
    def overlaps(self, other: 'Range') -> bool:
        if self.start <= other.start <= self.stop:
            return True
        if other.start <= self.start <= other.stop:
            return True
        return False

@dataclass
class Pair:
    ranges: tuple[Range, Range]

    @classmethod
    def from_line(cls, line: str) -> 'Pair':
        ranges = tuple(Range.from_string(half) for half in line.split(','))
        return cls(ranges)

    def overlaps(self) -> bool:
        if self.ranges[0].overlaps(self.ranges[1]):
            return True
        return False


filename = sys.argv[1]
lines = Path(filename).read_text().strip().split('\n')
pairs = [Pair.from_line(line) for line in lines]
overlapping_pairs = list(filter(lambda p: p.overlaps(), pairs))
n_pairs = len(overlapping_pairs)
print(n_pairs)