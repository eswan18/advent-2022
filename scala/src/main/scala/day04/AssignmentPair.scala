package day04

final case class AssignmentPair(assignments: (Assignment, Assignment))

object AssignmentPair:
    def fromString(s: String): AssignmentPair =
        val Array(a, b) = s.split(",")
        AssignmentPair((Assignment.fromString(a), Assignment.fromString(b)))

final case class Assignment(start: Int, end: Int):
    def contains(a: Assignment): Boolean =
        start <= a.start && end >= a.end
    
    def overlaps(a: Assignment): Boolean =
        start <= a.end && end >= a.start

object Assignment:
    def fromString(s: String): Assignment =
        val Array(start, end) = s.split("-")
        Assignment(start.toInt, end.toInt)
