package day05

import scala.util.matching.Regex

final case class Crate(letter: Char):
    override def toString: String = s"[${letter}]"

final case class CrateStack(stack: Vector[Crate]):
    override def toString: String =
        stack.map { c => c.toString }.mkString(" ")

final case class CrateGrid(stacks: Vector[CrateStack]):
    override def toString: String = 
        stacks.map { stack =>
            stack.toString
        }.mkString("\n")
    
    def move(fromStack: Int, toStack: Int, count: Int): CrateGrid =
        // Move the last `count` crates from `fromStack` and append to `toStack`
        val from = stacks(fromStack)
        val to = stacks(toStack)
        val removedCrates = from.stack.takeRight(count)
        val newFrom = CrateStack(from.stack.dropRight(count))
        val newTo = CrateStack(to.stack ++ removedCrates)
        CrateGrid(stacks.updated(fromStack, newFrom).updated(toStack, newTo))

    
    def takeInstructionOneByOne(instruction: Instruction): CrateGrid =
        // Repeatedly move as many times as instruction.count, each time moving one crate from instruction.fromStack to instruction.toStack
        var newGrid = this
        (1 to instruction.count).foreach { _ =>
            newGrid = newGrid.move(instruction.fromStack, instruction.toStack, 1)
        }
        newGrid

    
    def topCrates: Vector[Char] =
        stacks.map { stack =>
            stack.stack.lastOption match
                case Some(c) => c.letter
                case None => ' '
        }.filter {
            c => c != ' '
        }

object CrateGrid:
    def fromString(s: String) =
        val lines = s.split("\n").reverse.toVector
        // Figure out how many stacks there are based on the first line
        val nStacks = lines(0).split(" ").last.toInt
        var stacks = Vector.fill(nStacks)(CrateStack(Vector.empty))
        // Work through each line from the bottom, adding crates as needed.
        lines.tail.foreach(line => {
            stacks.zipWithIndex.foreach((stack, i) => {
                val index = 4 * i + 1
                line(index) match
                    case ' ' => ()
                    case c => stacks = stacks.updated(i, CrateStack(stack.stack :+ Crate(c)))
            })
        })
        CrateGrid(stacks)

final case class GridWithInstructions(var grid: CrateGrid, instructions: Vector[Instruction]):
    override def toString =
        grid.toString + "\n\n" + instructions.map { i => i.toString }.mkString("\n")

    def executeOneByOne(): Unit =
        // Execute all the instructions against the grid)
        println(grid)
        instructions.foreach { i => println(i); grid = grid.takeInstructionOneByOne(i); println(grid); () }

object GridWithInstructions:
    def fromString(s: String): GridWithInstructions =
        val Array(grid, instructions) = s.split("\n\n")
        GridWithInstructions(
            CrateGrid.fromString(grid),
            instructions.split("\n").toVector.map(Instruction.fromString(_))
        )

final case class Instruction(count: Int, fromStack: Int, toStack: Int):
    override def toString =
        s"Move $count crates from stack $fromStack to stack $toStack (0 indexed)"

object Instruction:
    def fromString(s: String): Instruction =
        val pattern: Regex = "move (\\d+) from (\\d+) to (\\d+)".r
        val (count, fromStack, toStack) = s match {
            case pattern(count, fromStack, toStack) =>
                (count, fromStack, toStack)
            case _ =>
                throw new Exception("No match")
        }
        // Adjust for 1-based indexing
        Instruction(count.toInt, fromStack.toInt-1, toStack.toInt-1)