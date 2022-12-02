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
}
outcome_mapping = {
    'X': 2,
    'Y': 0,
    'Z': 1,
}
m = {
    ROCK: 'rock',
    PAPER: 'paper',
    SCISSORS: 'scissors',
}

@dataclass
class Round:
    their_play: int
    outcome: int  # Represented as (our_play - their_play) % 3
    line: str

    def what_should_we_play(self) -> int:
        if self.outcome == 0:
            return self.their_play
        elif self.outcome == 2:  # need to lose
            our_play = self.their_play - 1
            if our_play == 0:
                our_play = 3
            return our_play
        elif self.outcome == 1:  # need to win
            our_play = self.their_play + 1
            if our_play == 4:
                our_play = 1
        return our_play
    
    def diagnostic(self):
        todo = 'lose' if self.outcome == 2 else 'win' if self.outcome == 1 else 'draw'
        print(f'they played {m[self.their_play]} and you need to {todo}')
        print(f"You'll play {m[self.what_should_we_play()]}")

    def score(self):
        self.diagnostic()
        score = 0
        if self.outcome == 2:  # loss
            ...
        elif self.outcome == 0:  # draw
            score += 3
        elif self.outcome == 1:  # win
            score += 6
        our_play = self.what_should_we_play()
        score += our_play
        return score


def parse_line(line: str) -> Round:
    theirs, outcome = line.split(' ')
    their_play = char_mapping[theirs]
    outcome = outcome_mapping[outcome]
    return Round(their_play, outcome, line)

rounds = [parse_line(line) for line in lines]
total_score = sum(round.score() for round in rounds)

print(total_score)