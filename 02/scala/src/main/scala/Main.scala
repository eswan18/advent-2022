package main

import scala.util.{Success,Failure,Try}

enum ABC:
    case A
    case B
    case C

object ABC:
    def withName(s: String): ABC = s match
        case "A" => A
        case "B" => B
        case "C" => C

enum XYZ:
    case X
    case Y
    case Z

object XYZ:
    def withName(s: String): XYZ = s match
        case "X" => X
        case "Y" => Y
        case "Z" => Z

case class Line(
    abc: ABC,
    xyz: XYZ,
)

enum Throw:
    case Rock, Paper, Scissors

case class ScenarioA(you: Throw, opp: Throw):
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


@main def main(args: String*): Unit =
  if args.length != 2 then
    println("Usage: sbt run [name]")
  val problem = args(0)
  val inputFile = args(1)
  val result = problem match
    case "a" => runA(inputFile)
    case "b" => runB(inputFile)
    case _ => Failure(new Exception(s"Unknown problem: $problem"))
  result match
    case Success(s) => println(s"Result: $s")
    case Failure(ex) => println(s"Error: ${ex.getMessage}")

def runA(inputFile: String): Try[Int] =
    for
        lines <- linesFromFile(inputFile)
        scenarios = lines.map(ScenarioA.fromLine)
        scores = scenarios.map(_.score())
    yield scores.sum

def runB(inputFile: String): Try[Int] =
    val lines = for
      lines <- linesFromFile(inputFile)
    yield lines
    Success(0)

def linesFromFile(inputFile: String): Try[Seq[Line]] =
  Try {
    val rawLines = scala.io.Source.fromFile(inputFile).getLines.toSeq
    rawLines.map(_.split(" ")).map {
      case Array(abc, xyz) => Line(ABC.withName(abc), XYZ.withName(xyz))
    }
  }