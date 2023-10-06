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


final case class Rucksack(items: Vector[Char]):
    def intersection(r: Rucksack): Vector[Char] =
        items.intersect(r.items).distinct
    
    def intersection(chars: Vector[Char]): Vector[Char] =
        items.intersect(chars).distinct

case object Rucksack:
    def fromLine(line: String): Try[Rucksack] =
        Try {
            // Split the line into sequences of characters, each of which represents an item
            val items = line.split("").map(_.charAt(0))
            Rucksack(items.toVector)
        }


def charPriority(c: Char): Try[Int] =
    c match
        case a if 'a' to 'z' contains a => Success(a.toInt - 96)
        case a if 'A' to 'Z' contains a => Success(a.toInt - 64 + 26)
        case _ => Failure(new Exception(s"Invalid character: $c"))