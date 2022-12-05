import sys
import re
from pathlib import Path
from itertools import takewhile

class Board:
    def __init__(self, n_columns: int):
        self.stacks: list[list[str]] = [[] for _ in range(n_columns)]
    
    def add_crate(self, column: int, crate: str):
        self.stacks[column].append(crate)
    
    def move_one(self, old_column: int, new_column: int):
        '''Move a crate from one stack to another.'''
        crate = self.stacks[old_column].pop()
        self.stacks[new_column].append(crate)
    
    def tops(self) -> str:
        '''Return the crates at the top of each column as a single string.'''
        tops = []
        for stack in self.stacks:
            if len(stack) > 0:
                tops.append(stack[-1])
        return ''.join(tops)
    
    def __repr__(self) -> str:
        return '\n'.join(f'{i+1}:{stack!r}' for i, stack in enumerate(self.stacks))

filename = sys.argv[1]
lines = Path(filename).read_text().split('\n')

# Lines up to the first newline are the starting positions.
position_lines = takewhile(lambda l: len(l) > 0, lines)
position_lines = list(position_lines)
*crate_lines, column_line = position_lines
# The number of columns should be the last non-blank character(s)
n_columns = column_line.strip().split(' ')[-1]
n_columns = int(n_columns)

board = Board(n_columns)
# Reverse order so you can start at the bottom.
for line in reversed(crate_lines):
    for column in range(n_columns):
        # What position should we look in to find this column's crate?
        position = 4 * column + 1
        # Check if we're done processing this line.
        if position >= len(line):
            break
        crate = line[position]
        if crate != ' ':
            board.add_crate(column, crate)

# Find lines that describe "moves".
move_start = len(position_lines) + 1
moves = lines[move_start:]

def execute_move(line: str):
    if len(line) == 0:
        return
    match = re.match('move (.*) from (.*) to (.*)', line)
    count, old_column, new_column = match.groups()
    count = int(count)
    # Adjust for 0-based indexing
    old_column = int(old_column) - 1
    new_column = int(new_column) - 1

    for _ in range(count):
        board.move_one(old_column, new_column)

for move in moves:
    execute_move(move)

print(board.tops())