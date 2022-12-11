import sys
from dataclasses import dataclass
from pathlib import Path

@dataclass
class Monkey:
    number: int  # What number monkey is this?
    items: list[int]  # Items the monkey is holding.
    operation: str  # Store the operation as a string to be `eval`ed later.
    divisor: int  # The number to divide by as part of our "test"
    on_true: int  # What number monkey to throw an item to if the test is true.
    on_false: int  # What number monkey to throw an item to if the test is false.


    @classmethod
    def from_section(cls, section: str) -> 'Monkey':
        lines = section.split('\n')
        # Sorry for ugly parsing; should use a regex but it would take longer to set up.
        number = int(lines[0].split(' ')[-1][:-1])
        item_list = lines[1].split(':')[1]
        items = [int(x) for x in item_list.split(', ')]
        operation = lines[2].split(' new = ')[1]
        divisor = int(lines[3].split(' divisible by ')[1])
        on_true = int(lines[4].split(' ')[-1])
        on_false = int(lines[5].split(' ')[-1])
        return cls(number, items, operation, divisor, on_true, on_false)
 

filename = sys.argv[1]
sections = Path(filename).read_text().strip().split('\n\n')
monkeys = [Monkey.from_section(section) for section in sections]
print(monkeys)