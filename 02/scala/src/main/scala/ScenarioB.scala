package main

enum Outcome:
    case Win, Lose, Draw
    def score(): Int = this match
        case Lose => 0
        case Draw => 3
        case Win => 6

final case class ScenarioB(opp: Throw, outcome: Outcome):
    def yourChoice(): Throw = (opp, outcome) match
        case (Throw.Rock, Outcome.Win) => Throw.Paper
        case (Throw.Rock, Outcome.Draw) => Throw.Rock
        case (Throw.Rock, Outcome.Lose) => Throw.Scissors
        case (Throw.Paper, Outcome.Win) => Throw.Scissors
        case (Throw.Paper, Outcome.Draw) => Throw.Paper
        case (Throw.Paper, Outcome.Lose) => Throw.Rock
        case (Throw.Scissors, Outcome.Win) => Throw.Rock
        case (Throw.Scissors, Outcome.Draw) => Throw.Scissors
        case (Throw.Scissors, Outcome.Lose) => Throw.Paper
    def score(): Int =
        val throwScore = yourChoice() match
            case Throw.Rock => 1
            case Throw.Paper => 2
            case Throw.Scissors => 3
        throwScore + outcome.score()

object ScenarioB:
    def fromLine(line: Line): ScenarioB =
        ScenarioB(
            opp = line.abc match
                case ABC.A => Throw.Rock
                case ABC.B => Throw.Paper
                case ABC.C => Throw.Scissors,
            outcome = line.xyz match
                case XYZ.X => Outcome.Lose
                case XYZ.Y => Outcome.Draw
                case XYZ.Z => Outcome.Win,
        )