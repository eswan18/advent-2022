package day07

import scala.util.{Try, Success, Failure}

val totalSpace = 70_000_000
val neededUnusedSpace = 30_000_000

def runB(input: String): Try[String] = 
    val parser = CommandParser(input)
    val commands = parser.parse()
    val fs = Filesystem.fromCommands(commands)
    val freeSpace = totalSpace - fs.size
    val spaceToFree = neededUnusedSpace - freeSpace
    println(spaceToFree)
    val viableDirs = fs.directories.map(dir => (dir, dir.size)).filter(_._2 >= spaceToFree).sortBy(_._2)
    val smallestDir = viableDirs.head
    Success(smallestDir._1.size.toString)

