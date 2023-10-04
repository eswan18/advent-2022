package main

import scala.util.{Try, Success, Failure}

// A rucksack is a bag with exactly two compartments
final case class CompartmentRucksack(compartments: (Vector[Char], Vector[Char])):
    def intersection(): Try[Char] =
        val intersection = compartments._1.intersect(compartments._2)
        Try(intersection(0))
    
    def intersectionPriority(): Try[Int] =
        val item = intersection()
        item match
            case Success(i) => i match
                case a if 'a' to 'z' contains a => { println(a.toInt - 96); Success(a.toInt - 96) }
                case a if 'A' to 'Z' contains a => { println(a.toInt - 64 + 26); Success(a.toInt - 64 + 26) }
            case Failure(f) => Failure(f)

case object CompartmentRucksack:
    def fromLine(line: String): Try[CompartmentRucksack] =
        Try {
            // Split the line into sequences of characters, each of which represents an item
            val items = line.split("").map(_.charAt(0))
            // split the items into two groups
            val (group1, group2) = items.splitAt(items.length / 2)
            CompartmentRucksack((group1.toVector, group2.toVector))
        }