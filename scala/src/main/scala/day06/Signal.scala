package day06

import scala.util.control.Breaks._

final case class Signal(content: Vector[Char]):
    def findStartOfPacketMarker: Option[Int] =
        var window = (0, 4)
        var marker: Option[Int] = None
        breakable {
            while window._2 <= content.length do
                val substring = content.slice(window._1, window._2)
                if substring.distinct.length == 4 then
                    marker = Some(window._2)
                    break
                window = (window._1 + 1, window._2 + 1)
        }
        marker

    def findStartOfMessageMarker: Option[Int] =
        var window = (0, 14)
        var marker: Option[Int] = None
        breakable {
            while window._2 <= content.length do
                val substring = content.slice(window._1, window._2)
                if substring.distinct.length == 14 then
                    marker = Some(window._2)
                    break
                window = (window._1 + 1, window._2 + 1)
        }
        marker