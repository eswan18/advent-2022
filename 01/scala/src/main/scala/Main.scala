package main

import scala.io.Source
import scala.util.{Try, Success, Failure, Using}

@main def main(args: String*): Unit = 
  if args.length != 1 then
    println("Usage: sbt run [name]")
  
  val inputFile = args(0)
  val result = run(inputFile)
  result match
    case Success(s) => println(s"Result: $s")
    case Failure(ex) => println(s"Error: ${ex.getMessage}")


def run(inputFile: String): Try[Integer] = 
  val lines = linesOfFile(inputFile) match
    case Success(lines) => lines
    case Failure(ex) => return Failure(ex)
  // group the lines, breaking when a blank line is found
  val groups = lines.foldLeft(List(List[String]())) { (acc, line) =>
    if line.isEmpty then
      List[String]() :: acc
    else
      (line :: acc.head) :: acc.tail
  }
  val sums = groups.map(_.map(_.toInt).sum)
  Try { sums.max }


def linesOfFile(filename: String): Try[List[String]] =
  Try {
    Using(Source.fromFile(filename)) { source => source.getLines().toList }.get
  }