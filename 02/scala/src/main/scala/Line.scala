package main

enum ABC:
    case A
    case B
    case C

object ABC:
    def withName(s: String): ABC = s match
        case "A" => A
        case "B" => B
        case "C" => C

enum XYZ:
    case X
    case Y
    case Z

object XYZ:
    def withName(s: String): XYZ = s match
        case "X" => X
        case "Y" => Y
        case "Z" => Z

final case class Line(abc: ABC, xyz: XYZ)