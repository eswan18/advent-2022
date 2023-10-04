package main

import scala.util.{Try, Success, Failure}

@main def main(args: String*): Int =
    if args.length != 2 then
        println("Usage: sbt run [a/b] [name]")
        return 1

    val input = Try {
      scala.io.Source.fromFile(args(1)).mkString
    } match {
      case Failure(exception) => throw exception
      case Success(input) => input
    }

    val result = args(0) match
        case "a" => runA(input)
        case "b" => Failure(new Exception("Not implemented"))
        case problem => Failure(new Exception(s"Unknown problem: $problem"))
    
    result match
        case Success(s) => println(s"Result: $s")
        case Failure(ex) => println(s"Error: ${ex.getMessage}")

    0