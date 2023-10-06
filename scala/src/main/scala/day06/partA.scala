package day06

import scala.util.{Try, Success, Failure}

def runA(input: String): Try[String] =
    val lines = input.split("\n").map(_.toCharArray().toVector)
    val line = lines(0)
    val result = Signal(line).findStartOfPacketMarker.get.toString
    Success(result)