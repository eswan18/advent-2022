import sys
from pathlib import Path
from dataclasses import dataclass


filename = sys.argv[1]
lines = Path(filename).read_text().strip().split('\n')

@dataclass
class Grid:
    rows: list[list[int]]

    def __getitem__(self, coords: tuple[int, int]) -> int:
        row, column = coords
        return self.rows[row][column]
    
    @property
    def shape(self) -> tuple[int, int]:
        n_rows = len(self.rows)
        n_cols = len(self.rows[0])
        return (n_rows, n_cols)
    
    def is_visible(self, row: int, col: int) -> bool:
        n_rows, n_cols = self.shape
        height = self[row, col]
        # Trees to the right.
        if all(self[row, c] < height for c in range(col+1, n_cols)):
            return True
        # Trees to the left
        if all(self[row, c] < height for c in range(0, col)):
            return True
        # Trees above.
        if all(self[r, col] < height for r in range(row+1, n_rows)):
            return True
        # Trees to the left
        if all(self[r, col] < height for r in range(0, row)):
            return True



grid = Grid([[int(char) for char in line] for line in lines])

# Brute foce: look at every element and see if it has a clear path to the edge.
n_rows, n_cols = grid.shape
n_visible_trees = 0
for r in range(n_rows):
    for c in range(n_cols):
        if grid.is_visible(r, c):
            n_visible_trees += 1
print(n_visible_trees)
