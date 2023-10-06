package main

import scala.util.{Try, Success, Failure}

def runA(input: String): Try[String] =
    val lines = input.split("\n")
    val tryRucksacks = lines.map(l => CompartmentRucksack.fromLine(l))
    val rucksacks = tryRucksacks.foldLeft(Try(Array.empty[CompartmentRucksack])) {
        (accTry, currTry) =>
        for {
            acc <- accTry
            curr <- currTry
        } yield acc :+ curr
    } match {
        case Failure(exception) => return Failure(exception)
        case Success(value) => value
    }

    val result = rucksacks.map{ r => r.intersectionPriority().get }.sum.toString()
    Success(result)
        