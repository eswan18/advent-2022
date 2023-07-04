#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

struct Rock {
    points: Vec<Point>,
}

impl Rock {
    fn build_from_text(text: &str) -> Rock {
        let mut points = Vec::new();
        for (line_number, line) in text.lines().enumerate() {
            let chars: Vec<char> = line.chars().collect();
            for (char_number, c) in chars.iter().enumerate() {
                if *c == '#' {
                    points.push(Point { x: char_number as i32, y: line_number as i32 });
                }
            }
        }
        Rock{ points }
    }

    fn build_multiple_from_text(text: &str) -> Vec<Rock> {
        let text_blocks = text.trim().split("\n\n");
        println!("{:?}", text_blocks);
        text_blocks.map(|block| Rock::build_from_text(block)).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_from_text() {
        let text = "###";
        let rock = Rock::build_from_text(text);
        let expected_points = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
        ];
        assert_eq!(rock.points, expected_points);

        let text = ".#.\n###\n.#.";
        let rock = Rock::build_from_text(text);
        let expected_points = vec![
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 2 },
        ];
        assert_eq!(rock.points, expected_points);

        let text = "#\n#\n#\n#";
        let rocket = Rock::build_from_text(text);
        let expected_points = vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 0, y: 3 },
        ];
        assert_eq!(rocket.points, expected_points);
    }

    #[test]
    fn test_build_multiple_from_text() {
        let text = "
####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";
        let rocks = Rock::build_multiple_from_text(text);
        assert_eq!(rocks.len(), 5);
    }
}