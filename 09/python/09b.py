import sys
from dataclasses import dataclass
from pathlib import Path
from collections import namedtuple
from typing import NamedTuple


LOG = False
N_KNOTS = 10

# A position on a grid
@dataclass
class Position:
    x: int
    y: int

    def vector_from(self, other: 'Position') -> tuple[int, int]:
        '''The vector representing how to get to this position from another.'''
        return (other.x - self.x, other.y - self.y)

    def adjacent_to(self, other: 'Position') -> bool:
        x = self.x - other.x
        y = self.y - other.y
        if abs(x) <= 1 and abs(y) <= 1:
            return True
        return False
    
    def as_tuple(self) -> tuple[int, int]:
        return self.x, self.y

filename = sys.argv[1]
lines = Path(filename).read_text().strip().split('\n')

class Grid:
    def __init__(self, head: Position | None = None):
        if head is None:
            head = Position(0, 0)
        self.head = head
        self.tail = head  # The tail starts at the same place as the head.
        # Track all the places the tail has been.
        self.trail: list[Position] = [head]
 
    def move_head(self, x: int, y: int):
        new_head = Position(self.head.x + x, self.head.y + y)
        self.head = new_head
        if LOG:
            print(f'head at {new_head}')

    def move_head_to(self, p: Position):
        self.head = p
        if LOG:
            print(f'head at {new_head}')
    
    def update_tail(self):
        if self.head.adjacent_to(self.tail):
            return
        vector = self.head.vector_from(self.tail)
        old_x, old_y = self.tail.x, self.tail.y
        match vector:
            case (0, dy) if dy < 0:
                self.tail = Position(old_x, old_y + 1)
            case (0, dy) if dy > 0:
                self.tail = Position(old_x, old_y - 1)
            case (dx, 0) if dx < 0:
                self.tail = Position(old_x + 1, old_y)
            case (dx, 0) if dx > 0:
                self.tail = Position(old_x - 1, old_y)
            case (dx, dy) if dx > 0 and dy > 0:
                # Diagonal up-right
                self.tail = Position(old_x - 1, old_y - 1)
            case (dx, dy) if dx > 0 and dy < 0:
                # Diagonal down-right
                self.tail = Position(old_x - 1, old_y + 1)
            case (dx, dy) if dx < 0 and dy > 0:
                # Diagonal up-left
                self.tail = Position(old_x + 1, old_y - 1)
            case (dx, dy) if dx < 0 and dy < 0:
                # Diagonal down-left
                self.tail = Position(old_x + 1, old_y + 1)
        if LOG:
            print(f'New tail: {self.tail}')


        if len(self.trail) == 0 or self.trail[-1] != self.tail:
            self.trail.append(self.tail)
        

grids = [Grid() for _ in range(N_KNOTS - 1)]
for line in lines:
    direction, distance = line.strip().split(' ')
    distance = int(distance)
    match direction:
        case 'R':
            move = (1, 0)
        case 'L':
            move = (-1, 0)
        case 'U':
            move = (0, 1)
        case 'D':
            move = (0, -1)
    for i in range(distance):
        # grids[0] is the head knot.
        grids[0].move_head(*move)
        grids[0].update_tail()
        if LOG:
            print('-------')
            print(f'Moving head to {grids[0].head}')
        for i in range(1, len(grids)):
            if LOG:
                print(f'Moving knot {i} to {grids[i-1].tail}')
            # Make the "head" of this rope the tail of the last
            grids[i].move_head_to(grids[i-1].tail)
            grids[i].update_tail()


trail = [t.as_tuple() for t in grids[-1].trail]
uniq_trail = set(trail)
if LOG:
    print(uniq_trail)
print(len(uniq_trail))