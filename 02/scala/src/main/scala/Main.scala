package main

import scala.util.{Success,Failure,Try}


@main def main(args: String*): Unit =
    println(args)
    if args.length != 2 then
        println("Usage: sbt run [a/b] [name]")
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
    for
      lines <- linesFromFile(inputFile)
      scores = lines.map(ScenarioB.fromLine).map(_.score())
    yield scores.sum

def linesFromFile(inputFile: String): Try[Seq[Line]] =
  Try {
    val rawLines = scala.io.Source.fromFile(inputFile).getLines.toSeq
    rawLines.map(_.split(" ")).map {
      case Array(abc, xyz) => Line(ABC.withName(abc), XYZ.withName(xyz))
    }
  }