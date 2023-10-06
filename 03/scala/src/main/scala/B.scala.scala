package main

import scala.util.{Try, Success, Failure}


def runB(input: String): Try[String] =
    val lines = input.split("\n")
    val tryRucksacks = lines.map(l => Rucksack.fromLine(l))
    val rucksacks = tryRucksacks.foldLeft(Try(Array.empty[Rucksack])) {
        (accTry, currTry) =>
        for {
            acc <- accTry
            curr <- currTry
        } yield acc :+ curr
    } match {
        case Failure(exception) => return Failure(exception)
        case Success(value) => value
    }

    // Group the rucksacks into threes
    val rucksackGroups = rucksacks.grouped(3).toVector
    val priorities = rucksackGroups.map { group => 
        for {
            overlap <- Try {
                val intersection = group(0).intersection(group(1).intersection(group(2)))
                if (intersection.length != 1) {
                    throw new Exception(s"Invalid input: rucksacks do not overlap in exactly one place. Found ${intersection}")
                }
                intersection(0)
            }
            priority <- charPriority(overlap)
        } yield priority
    }
    val totalPriority = priorities.collect { case Success(value) => value }.sum
    if (priorities.exists(_.isFailure)) {
        // find the failure
        val failure = priorities.find(_.isFailure).get
        failure match { case Failure(exception) => Failure(exception) }
    } else {
        Success(totalPriority.toString)
    }