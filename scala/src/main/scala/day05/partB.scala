package day05

import scala.util.{Try, Success, Failure}

def runB(input: String): Try[String] =
    val grid = GridWithInstructions.fromString(input)
    grid.executeInBulk()
    Success(grid.grid.topCrates.mkString)
