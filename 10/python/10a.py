import sys
from pathlib import Path

filename = sys.argv[1]
lines = Path(filename).read_text().strip().split('\n')

# our register
r = 1
# the number cycle we're on
cycle = 0
def tick() -> bool:
    global cycle
    cycle += 1
    should_add = False
    if (cycle - 20) % 40 == 0:
        should_add = True
    return should_add

result = 0

for line in lines:
    should_add = tick()
    if should_add:
        print(cycle*r)
        result += cycle * r
    line = line.strip().split()
    match line:
        case ["noop"]:
            continue
        case ["addx", x]:
            v = int(x)
            should_add = tick()
            if should_add:
                result += cycle * r
            r += v
        case _:
            raise RuntimeError('Unexpected input')

print(result)