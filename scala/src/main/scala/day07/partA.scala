package day07

import scala.util.{Try, Success, Failure}

def runA(input: String): Try[String] = 
    val parser = CommandParser(input)
    val commands = parser.parse()
    val fs = Filesystem.fromCommands(commands)
    val result = fs.directories.map(_.size).filter(_ <= 100_000).sum[Int]
    Success(result.toString)
