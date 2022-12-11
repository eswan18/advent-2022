import sys
from dataclasses import dataclass
from pathlib import Path
from functools import reduce
from operator import xor

current_id = 0
items: list['Item'] = []

class Item:
    def __init__(self, priority: int):
        global current_id
        self.priority = priority
        self.id = current_id
        current_id += 1
        items.append(self)
    
    def __repr__(self) -> str:
        return f'Item({self.priority}, id={self.id})'


@dataclass(frozen=True)
class State:
    owners_and_items: dict[int, tuple[int, ...]]
    inspections: tuple[int, ...]

    def __hash__(self) -> int:
        h = 0
        for owner, items in self.owners_and_items.items():
            item_hash = hash(items)
            h += owner * item_hash
        return h


@dataclass
class Monkey:
    number: int  # What number monkey is this?
    items: list[Item]  # Items the monkey is holding.
    operation: str  # Store the operation as a string to be `eval`ed later.
    divisor: int  # The number to divide by as part of our "test"
    on_true: int  # What number monkey to throw an item to if the test is true.
    on_false: int  # What number monkey to throw an item to if the test is false.
    _inspections: int = 0  # How many items has this monkey inspected?

    def __str__(self) -> str:
        item_str = ", ".join(repr(item) for item in self.items)
        return f'Monkey {self.number}: {item_str}'

    def inspect_items(self) -> list[tuple[Item, int]]:
        to_move = []
        while len(self.items) > 0:
            to_move.append(self.inspect_first())
        return to_move

    def inspect_first(self) -> tuple[Item, int]:
        self._inspections += 1
        item = self.items.pop(0)

        item.priority = eval(self.operation, {'old': item.priority})
        item.priority = item.priority // 3

        if (item.priority % self.divisor) == 0:
            return (item, self.on_true)
        else:
            return (item, self.on_false)


    @classmethod
    def from_section(cls, section: str) -> 'Monkey':
        lines = section.split('\n')
        # Sorry for ugly parsing; should use a regex but it would take longer to set up.
        number = int(lines[0].split(' ')[-1][:-1])
        item_list = lines[1].split(':')[1]
        items = [Item(int(x)) for x in item_list.split(', ')]
        operation = lines[2].split(' new = ')[1]
        divisor = int(lines[3].split(' divisible by ')[1])
        on_true = int(lines[4].split(' ')[-1])
        on_false = int(lines[5].split(' ')[-1])
        return cls(number, items, operation, divisor, on_true, on_false)
 

filename = sys.argv[1]
sections = Path(filename).read_text().strip().split('\n\n')
monkeys = [Monkey.from_section(section) for section in sections]

N_ROUNDS = 600
states: dict[int, State] = {}
for i in range(N_ROUNDS):
    for monkey in monkeys:
        to_move = monkey.inspect_items()
        for item, recipient in to_move:
            monkeys[recipient].items.append(item)
    # Record the state of who holds which items.
    state = State(
        {
            monkey.number: tuple(sorted(item.id for item in monkey.items))
            for monkey in monkeys
        },
        tuple(m._inspections for m in monkeys),
    )
    # print(state.owners_and_items)
    if hash(state) in states:
        ...
        # print("We've seen this before!")
    if hash(state) == 1679267220673506760:
        print(i)

    states[hash(state)] = state


counts = [m._inspections for m in monkeys]
counts = sorted(counts)

second_highest, highest = counts[-2:]
print(second_highest * highest)