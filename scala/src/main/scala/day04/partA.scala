package day04

import scala.util.{Try, Success, Failure}

def runA(input: String): Try[String] =
    val pairs = input.split("\n").map(AssignmentPair.fromString).toVector
    val containCount = pairs.map { pair =>
        val (a, b) = pair.assignments
        a.contains(b) || b.contains(a)
    }.count(identity)
    Success(containCount.toString)
