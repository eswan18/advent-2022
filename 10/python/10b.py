import sys
from pathlib import Path

filename = sys.argv[1]
lines = Path(filename).read_text().strip().split('\n')

# our register
r = 1
# the number cycle we're on
cycle = 0
# one continuous string representing all drawn pixels
pixels = ''

def draw():
    global cycle, pixels
    position = cycle % 40
    if abs(r - position) <= 1:
        pixels += '#'
    else:
        pixels += '.'
    cycle += 1

result = 0

for line in lines:
    draw()
    line = line.strip().split()
    match line:
        case ["noop"]:
            continue
        case ["addx", x]:
            v = int(x)
            draw()
            r += v
        case _:
            raise RuntimeError('Unexpected input')

# Break pixels into six lines.
n_lines = 6
line_length = len(pixels) / n_lines
for i in range(n_lines):
    start_pos = int(i * line_length)
    end_pos = int((i + 1) * line_length)
    line = pixels[start_pos:end_pos]
    print(line)