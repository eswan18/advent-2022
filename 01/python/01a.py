import sys
from pathlib import Path
import itertools

filename = sys.argv[1]
lines = Path(filename).read_text().split('\n')

# Break up the lines into chunks based on whitespace
line_groups: list[list[str]] = []
while len(lines) > 0:
    if lines[0] == '':
        lines = lines[1:]
        continue
    next_group = itertools.takewhile(lambda line: len(line) > 0, lines)
    next_group = [int(line) for line in next_group]
    line_groups.append(next_group)
    # Move forward
    lines = lines[len(next_group):]

group_sums = (sum(group) for group in line_groups)
print(max(group_sums))