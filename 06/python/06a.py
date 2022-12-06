import sys
from pathlib import Path

filename = sys.argv[1]
buffer = Path(filename).read_text().strip()

slice_end = 4
result = -1
while slice_end <= len(buffer):
    slice_start = slice_end - 4
    window = buffer[slice_start:slice_end]
    if len(set(window)) == 4:
        result = slice_end
        break
    slice_end += 1

print(result)