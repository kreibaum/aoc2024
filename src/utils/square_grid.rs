//! Module for "square character grid" utility functions.
//! Square character grids are a common input type in Advent of Code.

use std::convert::TryInto;

/// This is generic over the coordinate type, because you often want to use
/// signed integers for coordinates instead of unsigned integers.
pub struct SquareCharacterGrid<Coord: num::PrimInt> {
    grid: Vec<Vec<char>>,
    w: Coord,
    h: Coord,
}

impl <Coord: num::PrimInt> SquareCharacterGrid<Coord> {
    /// Create a new SquareCharacterGrid from a string.
    pub fn new(input: &str) -> Self {
        let mut grid = vec![];
        let mut w = Coord::zero();
        let mut h = Coord::zero();
        for line in input.lines() {
            let mut row = vec![];
            for c in line.chars() {
                row.push(c);
            }
            grid.push(row);
            h = h + Coord::one();
            w = Coord::from(grid[0].len()).expect("width out of bounds");
        }
        Self { grid, w, h }
    }

    pub fn size(&self) -> (Coord, Coord) {
        (self.w, self.h)
    }

    /// Get an iterator over the grid.
    pub fn iter(&self) -> SquareCharacterGridIterator<Coord> {
        SquareCharacterGridIterator {
            grid: self,
            x: Coord::zero(),
            y: Coord::zero(),
        }
    }

    /// Get the character at a specific coordinate.
    pub fn get(&self, x: Coord, y: Coord) -> Option<char> {
        if x < Coord::zero() || y < Coord::zero() {
            return None;
        }
        let x = x.to_usize().expect("x coordinate out of usize bounds");
        let y = y.to_usize().expect("y coordinate out of usize bounds");
        if x >= self.grid[0].len() || y >= self.grid.len() {
            return None;
        }
        Some(self.grid[y][x])
    }

    /// Set the character at a specific coordinate.
    pub fn set(&mut self, x: Coord, y: Coord, c: char) {
        let x = x.to_usize().expect("x coordinate out of usize bounds");
        let y = y.to_usize().expect("y coordinate out of usize bounds");
        self.grid[y][x] = c;
    }

    /// Print the grid to the console.
    pub fn print(&self) {
        for row in &self.grid {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
    }
}

pub struct SquareCharacterGridIterator<'a, Coord: num::PrimInt> {
    grid: &'a SquareCharacterGrid<Coord>,
    x: Coord,
    y: Coord,
}

impl <'a, Coord: num::PrimInt> Iterator for SquareCharacterGridIterator<'a, Coord> {
    type Item = (Coord, Coord, char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.h {
            return None;
        }
        let c = self.grid.get(self.x, self.y).expect("iterator out of bounds");
        let result = (self.x, self.y, c);

        // Step one forward, check for linebreaks.
        self.x = self.x + Coord::one();
        if self.x >= self.grid.w {
            self.x = Coord::zero();
            self.y = self.y + Coord::one();
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_character_grid() {
        let input = "..#\n#..\n...";
        let grid = SquareCharacterGrid::new(input);
        assert_eq!(grid.get(0, 0), Some('.'));
        assert_eq!(grid.get(1, 0), Some('.'));
        assert_eq!(grid.get(2, 0), Some('#'));
        assert_eq!(grid.get(0, 1), Some('#'));
        assert_eq!(grid.get(1, 1), Some('.'));
        assert_eq!(grid.get(2, 1), Some('.'));
        assert_eq!(grid.get(0, 2), Some('.'));
        assert_eq!(grid.get(1, 2), Some('.'));
        assert_eq!(grid.get(2, 2), Some('.'));

        assert_eq!(grid.w, 3);
        assert_eq!(grid.h, 3);
    }

    #[test]
    fn test_iterator() {
        let input = "..#\n#..\n...";
        let grid = SquareCharacterGrid::new(input);
        let mut iter = grid.iter();

        assert_eq!(iter.next(), Some((0, 0, '.')));
        assert_eq!(iter.next(), Some((1, 0, '.')));
        assert_eq!(iter.next(), Some((2, 0, '#')));

        assert_eq!(iter.next(), Some((0, 1, '#')));
        assert_eq!(iter.next(), Some((1, 1, '.')));
        assert_eq!(iter.next(), Some((2, 1, '.')));

        assert_eq!(iter.next(), Some((0, 2, '.')));
        assert_eq!(iter.next(), Some((1, 2, '.')));
        assert_eq!(iter.next(), Some((2, 2, '.')));
    }
}