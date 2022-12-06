import sys
from pathlib import Path

filename = sys.argv[1]
buffer = Path(filename).read_text().strip()

marker_len = 14

slice_end = marker_len
result = -1
while slice_end <= len(buffer):
    slice_start = slice_end - marker_len
    window = buffer[slice_start:slice_end]
    if len(set(window)) == marker_len:
        result = slice_end
        break
    slice_end += 1

print(result)