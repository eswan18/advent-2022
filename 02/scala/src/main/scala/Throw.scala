package main

enum Throw:
  case Rock, Paper, Scissors

  def score(): Int =
    this match
      case Throw.Rock => 0
      case Throw.Paper => 1
      case Throw.Scissors => 2
