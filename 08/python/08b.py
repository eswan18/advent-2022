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
        
    def scenic_score(self, row: int, col: int) -> int:
        n_rows, n_cols = self.shape
        height = self[row, col]

        right_score = 0
        for c in range(col+1, n_cols):
            right_score += 1
            if self[row, c] >= height:
                break

        left_score = 0
        for c in range(col-1, -1, -1):
            left_score += 1
            if self[row, c] >= height:
                break

        up_score = 0
        for r in range(row+1, n_rows):
            up_score += 1
            if self[r, col] >= height:
                break
            
        down_score = 0
        for r in range(row-1, -1, -1):
            down_score += 1
            if self[r, col] >= height:
                break
        
        return right_score * left_score * up_score * down_score




grid = Grid([[int(char) for char in line] for line in lines])

# Brute foce: look at every element and see if it has a clear path to the edge.
n_rows, n_cols = grid.shape
best_scenic_score = -1
best_scenic_coords = None
for r in range(n_rows):
    for c in range(n_cols):
        score = grid.scenic_score(r, c)
        if score >= best_scenic_score:
            best_scenic_score = score
            best_scenic_coords = (r, c)

print(best_scenic_score)
print(best_scenic_coords)
