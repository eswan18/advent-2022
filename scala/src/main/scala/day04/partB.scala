package day04

import scala.util.{Try, Success, Failure}

def runB(input: String): Try[String] =
    val pairs = input.split("\n").map(AssignmentPair.fromString).toVector
    val containCount = pairs.map { pair =>
        val (a, b) = pair.assignments
        a.overlaps(b)
    }.count(identity)
    Success(containCount.toString)