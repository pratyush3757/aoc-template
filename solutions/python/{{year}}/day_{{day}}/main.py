import os.path
from pathlib import Path

with open(os.path.join(
    Path(os.path.dirname(__file__)).parents[3],
    'input', '{{year}}', 'day_{{day}}',
    'input.txt')) as f:
    lines = f.readlines()

# part1
def part_1():
    pass

# part2
def part_2():
    pass
