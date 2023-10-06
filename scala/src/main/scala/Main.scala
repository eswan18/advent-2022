package main

import scala.util.{Try, Success, Failure}
import day04._

@main def main(args: String*): Int =
    if args.length != 3 then
        println("Usage: sbt run [day] [a/b] [input_file]")
        return 1

    val input = Try {
      scala.io.Source.fromFile(args(2)).mkString
    } match {
      case Failure(exception) => throw exception
      case Success(input) => input
    }

    val result = (args(0), args(1)) match
        case ("4", "a") => day04.runA(input)
        case ("4", "b") => day04.runB(input)
        case ("5", "a") => day05.runA(input)
        case problem => Failure(new Exception("No such puzzle"))

    result match
        case Success(s) => println(s"Result: $s"); 0
        case Failure(ex) => println(s"Error: ${ex.getMessage}"); 1