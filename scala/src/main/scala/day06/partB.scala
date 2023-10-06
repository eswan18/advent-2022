package day06

import scala.util.{Try, Success, Failure}

def runB(input: String): Try[String] =
    val lines = input.split("\n").map(_.toCharArray().toVector)
    val line = lines(0)
    val result = Signal(line).findStartOfMessageMarker.get.toString
    Success(result)
