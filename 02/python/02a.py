import sys
from pathlib import Path
from dataclasses import dataclass

filename = sys.argv[1]
lines = Path(filename).read_text().split('\n')
# Filter out empty lines.
lines = (line for line in lines if len(line) > 0)

# A fake enum
ROCK = 1
PAPER = 2
SCISSORS = 3

char_mapping = {
    'A': ROCK,
    'B': PAPER,
    'C': SCISSORS,
    'X': ROCK, 
    'Y': PAPER,
    'Z': SCISSORS,
}

@dataclass
class Round:
    their_play: int
    our_play: int
    line: str

    def winner(self) -> str:
        mod = (self.our_play - self.their_play) % 3
        if mod == 0:
            return 'draw'
        elif mod == 1:
            return 'us'
        elif mod == 2:
            return 'them'
        else:
            raise RuntimeError

    def score(self):
        score = 0
        winner = self.winner()
        if winner == 'us':
            score += 6
        elif winner == 'draw':
            score += 3
        score += self.our_play
        return score


def parse_line(line: str) -> Round:
    theirs, ours = line.split(' ')
    our_play = char_mapping[ours]
    their_play = char_mapping[theirs]
    return Round(their_play, our_play, line)

rounds = [parse_line(line) for line in lines]
total_score = sum(round.score() for round in rounds)

print(total_score)