import sys
from dataclasses import dataclass
from pathlib import Path
from enum import Enum
from math import floor, sqrt
from functools import cache, reduce
from pprint import pprint

factors_to_check: list[int] = []

# @cache
# def simplify(p: int) -> int:
#     prod = reduce(lambda x, y: x * y, factors_to_check, 1)
#     print(prod)
#     remainder = 0
#     quotient, remainder = divmod(p, prod)
#     while remainder == 0:
#         p = quotient
#         quotient, remainder = divmod(p, prod)
#     return p

@cache
def true_factor(x: int) -> tuple[int]:
    for i in range(2, floor(sqrt(x)) + 1):
        quotient, remainder = divmod(x, i)
        if remainder == 0:
            return (i,) + true_factor(quotient)
    # If we got here, we found no factors.
    return (x,)


    
@cache
def factor(x: int) -> set[int]:
    # Check all relevant factors.
    for i in factors_to_check:
        quotient, remainder = divmod(x, i)
        if remainder == 0:
            return {i}.union(factor(quotient))
    # If we got here, we found no factors.
    return set()


class OperationType(Enum):
    addition = 1
    multiplication = 2
    square = 3


@dataclass
class Item:
    priority: int
    factors: set[int]

    def __post_init__(self):
        if not hasattr(self.__class__, 'instances'):
            self.__class__.instances = []
        self.__class__.instances.append(self)

    def update_factors(self):
        self.factors = factor(self.priority)

    @classmethod
    def from_string(cls, s: str) -> 'Item':
        i = int(s)
        # It's key that we don't call factor yet because results get cached.
        return cls(i, {})


@dataclass
class Monkey:
    number: int  # What number monkey is this?
    items: list[Item]  # Items the monkey is holding.
    operation_type: OperationType  # Do we add, multiply, square, etc
    operation_args: tuple[int, ...]  # What arguments to use in the operation
    divisor: int  # The number to divide by as part of our "test"
    on_true: int  # What number monkey to throw an item to if the test is true.
    on_false: int  # What number monkey to throw an item to if the test is false.
    _inspections: int = 0  # How many items has this monkey inspected?

    def inspect_items(self) -> list[tuple[Item, int]]:
        to_move = []
        while len(self.items) > 0:
            to_move.append(self.inspect_first())
        return to_move

    def inspect_first(self) -> tuple[Item, int]:
        self._inspections += 1
        item = self.items.pop(0)

        match self.operation_type:
            case OperationType.square:
                item.priority = item.priority * item.priority
                # No need to update factors; squaring doesn't change anything.
            case OperationType.addition:
                item.priority = item.priority + self.operation_args[0]
                # Recompute factors
                # Basically all of the time is spent here.
                item.factors = factor(item.priority)
            case OperationType.multiplication:
                for multiplier in self.operation_args:
                    item.priority = item.priority * multiplier
                    item.factors.add(multiplier)
            case _:
                raise RuntimeError('Unexpected operation type')
        # Try to simplify the number
        # I don't think this actually works.
        # item.priority = simplify(item.priority)

        if self.divisor in item.factors:
            return (item, self.on_true)
        else:
            return (item, self.on_false)


    @classmethod
    def from_section(cls, section: str) -> 'Monkey':
        lines = section.split('\n')
        # Sorry for ugly parsing; should use a regex but it would take longer to set up.
        number = int(lines[0].split(' ')[-1][:-1])
        item_list = lines[1].split(':')[1]
        items = [Item.from_string(x) for x in item_list.split(', ')]
        operation_string = lines[2].split(': new = ')[1]
        match operation_string.split(' '):
            case ['old', '*', 'old']:
                operation_type = OperationType.square
                args = ()
            case ['old', '+', addend]:
                a = int(addend)
                operation_type = OperationType.addition
                args = (a,)
            case ['old', '*', multiplier]:
                m = int(multiplier)
                operation_type = OperationType.multiplication
                args = tuple(true_factor(m))
            case _:
                raise RuntimeError('unexpected input')

        divisor = int(lines[3].split(' divisible by ')[1])
        factors_to_check.append(divisor)
        on_true = int(lines[4].split(' ')[-1])
        on_false = int(lines[5].split(' ')[-1])
        return cls(number, items, operation_type, args, divisor, on_true, on_false)
 

filename = sys.argv[1]
sections = Path(filename).read_text().strip().split('\n\n')
monkeys = [Monkey.from_section(section) for section in sections]
# The items initially computed their factors before we saw all divisors; do it again.
print(factors_to_check)
for item in Item.instances:
    item.update_factors()

N_ROUNDS = 1000
for i in range(N_ROUNDS):
    if i in (1, 20, 100, 250, 500, 600, 700, 800, 900, 1000):
        print(f'Round {i}')
        print([m._inspections for m in monkeys])
    for monkey in monkeys:
        to_move = monkey.inspect_items()
        for item, recipient in to_move:
            monkeys[recipient].items.append(item)

counts = [m._inspections for m in monkeys]
counts = sorted(counts)

second_highest, highest = counts[-2:]
print(second_highest * highest)