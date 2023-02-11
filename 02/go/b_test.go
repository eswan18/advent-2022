package main

import "testing"

func TestRoundSpecToRound(t *testing.T) {
	rs := roundSpec{
		opponent: Rock,
		outcome:  Draw,
	}
	r := rs.toRound()
	if r.opponent != rs.opponent {
		t.Fatal("Opponents don't match")
	}
	if r.you != rs.opponent {
		t.Fatalf("Your play is wrong; it's %d", r.you)
	}

	rs = roundSpec{
		opponent: Rock,
		outcome:  Lose,
	}
	r = rs.toRound()
	if r.opponent != rs.opponent {
		t.Fatal("Opponents don't match")
	}
	if r.you != Scissors {
		t.Fatalf("Your play is wrong; it's %d", r.you)
	}

	rs = roundSpec{
		opponent: Scissors,
		outcome:  Win,
	}
	r = rs.toRound()
	if r.opponent != rs.opponent {
		t.Fatal("Opponents don't match")
	}
	if r.you != Rock {
		t.Fatalf("Your play is wrong; it's %d", r.you)
	}

	rs = roundSpec{
		opponent: Scissors,
		outcome:  Lose,
	}
	r = rs.toRound()
	if r.opponent != rs.opponent {
		t.Fatal("Opponents don't match")
	}
	if r.you != Paper {
		t.Fatalf("Your play is wrong; it's %d", r.you)
	}
}
