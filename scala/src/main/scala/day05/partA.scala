package day05

import scala.util.{Try, Success, Failure}

def runA(input: String): Try[String] =
    val grid = GridWithInstructions.fromString(input)
    grid.executeOneByOne()
    Success(grid.grid.topCrates.mkString)