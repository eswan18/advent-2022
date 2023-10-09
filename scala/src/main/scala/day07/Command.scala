package day07

import scala.util.matching.Regex

sealed trait Command

final case class LS(listings: Vector[Item]) extends Command

final case class CD(path: String) extends Command

final case class CommandParser(input: String):
    val lines = input.split("\n").toVector
    var index = 0

    def parse(): Vector[Command] =
        var commands = Vector.empty[Command]
        while (index < lines.length) {
            val line = lines(index)
            index += 1
            // Lines are of the form `$ ls` or `$ cd <path>`. Use a regex to extract.
            val commandPattern = new Regex("""\$ (ls|cd)(.*)""")
            var command: Command = line match
                case commandPattern("ls", arg) => LS(Vector.empty)
                case commandPattern("cd", arg) => CD(arg.strip)
                case commandPattern(s*) => println(s); throw new Exception(s"Invalid command: $line")
                case _ => throw new Exception(s"Invalid command: $line")
            if command == LS(Vector.empty) then
                // The next lines are the listings.
                val listings = lines.drop(index).takeWhile(!_.startsWith("$")).map { listing =>
                    val parts = listing.split(" ")
                    parts match
                        case Array("dir", name) => Folder(name, Vector.empty)
                        case Array(size, name) => File(name, size.toInt)
                        case _ => throw new Exception(s"Invalid listing: $listing")
                }
                index += listings.length
                command = LS(listings)
            commands = commands :+ command
        }
        commands