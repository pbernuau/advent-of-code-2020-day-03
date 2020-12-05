use core::fmt;
use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();
    let geology = Geology::from_str(input.as_str()).unwrap();

    let mut n = 1;
    n = n * test_slope(&geology, 1, 1);
    n = n * test_slope(&geology, 1, 3);
    n = n * test_slope(&geology, 1, 5);
    n = n * test_slope(&geology, 1, 7);
    n = n * test_slope(&geology, 2, 1);

    println!("Product of trees encountered: {}", n);
}

fn test_slope(geology: &Geology, row_slope: usize, col_slope: usize) -> usize {
    let trees = count_trees_in_toboggan_path(geology, row_slope, col_slope);
    println!("Right {}, down {}: {} trees", col_slope, row_slope, trees);
    trees
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Square {
    Open,
    Tree,
}

struct Geology {
    grid: Box<[Square]>,
    width: usize,
    height: usize,
}

impl Geology {
    fn new(width: usize, height: usize) -> Geology {
        let grid = vec![Square::Open; width * height].into_boxed_slice();

        Geology {
            grid,
            width,
            height,
        }
    }

    fn get(&self, row: usize, col: usize) -> Square {
        self.grid[row * self.width + col]
    }

    fn set(&mut self, row: usize, col: usize, square: Square) {
        self.grid[row * self.width + col] = square
    }
}

impl fmt::Display for Geology {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                match self.get(row, col) {
                    Square::Open => { write!(fmt, ".")? }
                    Square::Tree => { write!(fmt, "#")? }
                }
            }
            write!(fmt, "\n")?
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct GeologyParsingError(String);

impl GeologyParsingError {
    fn new(msg: &str) -> GeologyParsingError {
        GeologyParsingError(String::from(msg))
    }
}

impl fmt::Display for GeologyParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid input: {}", self.0)
    }
}

impl FromStr for Geology {
    type Err = GeologyParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let width = bytes.iter().position(|&b| b == b'\n').ok_or(GeologyParsingError::new("missing end-of-line"))?;
        let height = bytes.len() / (width + 1);

        let mut geology = Geology::new(width, height);

        println!("width: {}, height: {}, len: {}", width, height, bytes.len());

        for row in 0..height {
            for col in 0..width {
                let byte = bytes
                    .get(row * (width + 1) + col)
                    .ok_or(GeologyParsingError::new("Invalid indexing"))?;

                let square = match byte {
                    b'.' => { Ok(Square::Open) }
                    b'#' => { Ok(Square::Tree) }
                    _ => { Err(GeologyParsingError::new("Invalid character")) }
                }?;

                geology.set(row, col, square);
            }
        }

        Ok(geology)
    }
}

fn build_toboggan_path(geology: &Geology, row_slope: usize, col_slope: usize) -> Path {
    let mut path = vec![];
    let mut col = 0;
    for row in (0..geology.height).step_by(row_slope) {
        path.push(Point { row, col });
        col = (col + col_slope) % geology.width
    }

    path
}

fn count_trees_in_toboggan_path(geology: &Geology, row_slope: usize, col_slope: usize) -> usize {
    build_toboggan_path(geology, row_slope, col_slope)
        .iter()
        .map(|&Point { row, col }| geology.get(row, col))
        .filter(|&square| square == Square::Tree)
        .count()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    row: usize,
    col: usize,
}

type Path = Vec<Point>;

#[cfg(test)]
mod tests {
    use crate::Square::{Open, Tree};

    use super::*;

    const INPUT: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

    const SQUARES: [Square; 11 * 11] = [
        Open, Open, Tree, Tree, Open, Open, Open, Open, Open, Open, Open,
        Tree, Open, Open, Open, Tree, Open, Open, Open, Tree, Open, Open,
        Open, Tree, Open, Open, Open, Open, Tree, Open, Open, Tree, Open,
        Open, Open, Tree, Open, Tree, Open, Open, Open, Tree, Open, Tree,
        Open, Tree, Open, Open, Open, Tree, Tree, Open, Open, Tree, Open,
        Open, Open, Tree, Open, Tree, Tree, Open, Open, Open, Open, Open,
        Open, Tree, Open, Tree, Open, Tree, Open, Open, Open, Open, Tree,
        Open, Tree, Open, Open, Open, Open, Open, Open, Open, Open, Tree,
        Tree, Open, Tree, Tree, Open, Open, Open, Tree, Open, Open, Open,
        Tree, Open, Open, Open, Tree, Tree, Open, Open, Open, Open, Tree,
        Open, Tree, Open, Open, Tree, Open, Open, Open, Tree, Open, Tree,
    ];

    #[test]
    fn test_geology_parsing() {
        let geology = Geology::from_str(INPUT).unwrap();

        assert_eq!(geology.height, 11);
        assert_eq!(geology.width, 11);
        assert_eq!(geology.grid.into_vec(), SQUARES[..].to_vec());
    }

    #[test]
    fn test_geology_display() {
        let geology = Geology::from_str(INPUT).unwrap();
        assert_eq!(geology.to_string(), String::from(INPUT));
    }

    #[test]
    fn test_build_toboggan_path() {
        let geology = Geology::from_str(INPUT).unwrap();
        let actual = build_toboggan_path(&geology, 1, 3);

        let expected = vec![
            Point { row: 0, col: 0 },
            Point { row: 1, col: 3 },
            Point { row: 2, col: 6 },
            Point { row: 3, col: 9 },
            Point { row: 4, col: 1 },
            Point { row: 5, col: 4 },
            Point { row: 6, col: 7 },
            Point { row: 7, col: 10 },
            Point { row: 8, col: 2 },
            Point { row: 9, col: 5 },
            Point { row: 10, col: 8 },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_count_trees_in_toboggan_path() {
        let geology = Geology::from_str(INPUT).unwrap();
        assert_eq!(count_trees_in_toboggan_path(&geology, 1, 3), 7);
    }
}