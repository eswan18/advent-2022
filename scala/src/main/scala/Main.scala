package main

import scala.util.{Try, Success, Failure}

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
        case ("5", "b") => day05.runB(input)
        case ("6", "a") => day06.runA(input)
        case ("6", "b") => day06.runB(input)
        case ("7", "a") => day07.runA(input)
        case ("7", "b") => day07.runB(input)
        case problem => Failure(new Exception("No such puzzle"))

    result match
        case Success(s) => println(s"Result: $s"); 0
        case Failure(ex) => println(s"Error: ${ex.getMessage}"); 1