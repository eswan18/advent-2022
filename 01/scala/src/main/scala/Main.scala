package main

import scala.io.Source
import scala.util.{Try, Success, Failure, Using}

@main def main(args: String*): Unit = 
  if args.length != 2 then
    println("Usage: sbt run [name]")
  val problem = args(0)
  val inputFile = args(1)
  val result = problem match
    case "a" => run_a(inputFile)
    case "b" => run_b(inputFile)
    case _ => Failure(new Exception(s"Unknown problem: $problem"))
  result match
    case Success(s) => println(s"Result: $s")
    case Failure(ex) => println(s"Error: ${ex.getMessage}")


def run_a(inputFile: String): Try[Integer] = 
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

def run_b(inputFile: String): Try[Integer] =
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
  // Order the groups by decreasing sum
  val orderedGroups = groups.sortBy(_.map(_.toInt).sum).reverse
  // Get the total of the first three groups
  Try {
    orderedGroups.take(3).map(_.map(_.toInt).sum).sum
  }

def linesOfFile(filename: String): Try[List[String]] =
  Try {
    Using(Source.fromFile(filename)) { source => source.getLines().toList }.get
  }