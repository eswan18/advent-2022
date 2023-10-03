package main

final case class ScenarioA(you: Throw, opp: Throw):
    def score(): Int =
        val throwScore = you match
            case Throw.Rock => 1
            case Throw.Paper => 2
            case Throw.Scissors => 3
        val outcomeScore = (you, opp) match
            case (Throw.Rock, Throw.Rock) => 3
            case (Throw.Rock, Throw.Paper) => 0
            case (Throw.Rock, Throw.Scissors) => 6
            case (Throw.Paper, Throw.Rock) => 6
            case (Throw.Paper, Throw.Paper) => 3
            case (Throw.Paper, Throw.Scissors) => 0
            case (Throw.Scissors, Throw.Rock) => 0
            case (Throw.Scissors, Throw.Paper) => 6
            case (Throw.Scissors, Throw.Scissors) => 3
        throwScore + outcomeScore


object ScenarioA:
    def fromLine(line: Line): ScenarioA =
        ScenarioA(
            opp = line.abc match
                case ABC.A => Throw.Rock
                case ABC.B => Throw.Paper
                case ABC.C => Throw.Scissors,
            you = line.xyz match
                case XYZ.X => Throw.Rock
                case XYZ.Y => Throw.Paper
                case XYZ.Z => Throw.Scissors,
        )