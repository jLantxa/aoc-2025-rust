#![allow(dead_code)]

use anyhow::{Result, bail};
use std::ops::{Index, IndexMut};

/// Defines the 8 cardinal and intercardinal directions (x_delta, y_delta).
/// Often used for pathfinding or neighbor checks.
pub(crate) const DIRECTIONS_8: [(isize, isize); 8] = [
    // Top row
    (-1, -1),
    (0, -1),
    (1, -1),
    // Middle row (excluding self)
    (-1, 0),
    (1, 0),
    // Bottom row
    (-1, 1),
    (0, 1),
    (1, 1),
];

/// Defines the 4 cardinal directions (x_delta, y_delta).
pub(crate) const DIRECTIONS_4: [(isize, isize); 4] = [
    (0, -1), // Up
    (0, 1),  // Down
    (-1, 0), // Left
    (1, 0),  // Right
];

/// A general-purpose 2D grid structure backed by a single `Vec<T>` in row-major order.
/// Coordinates are accessed as `(column, row)` or `(x, y)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Grid<T: Default + Copy> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

/// An iterator over the (i, j) coordinates of a Grid, yielding `(column, row)`.
#[derive(Debug, Clone)]
pub(crate) struct GridCoordsIterator {
    width: usize,
    height: usize,
    // The current column index (i)
    current_i: usize,
    // The current row index (j)
    current_j: usize,
}

impl Iterator for GridCoordsIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_j >= self.height {
            // End of iteration
            return None;
        }

        let result = (self.current_i, self.current_j);

        // Move to the next column
        self.current_i += 1;

        // If we reach the end of the row, reset column and move to the next row
        if self.current_i >= self.width {
            self.current_i = 0;
            self.current_j += 1;
        }

        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let total = self.width * self.height;
        let visited = self.current_j * self.width + self.current_i;
        let remaining = total.saturating_sub(visited);
        (remaining, Some(remaining))
    }
}

// Allows for efficient pre-allocation and counting
impl std::iter::ExactSizeIterator for GridCoordsIterator {}

/// An iterator over the (i, j) coordinates and immutable references (`&T`) of all elements.
pub(crate) struct GridIterator<'a, T> {
    width: usize,
    // Iterates over the coordinates
    coords_iter: GridCoordsIterator,
    // Iterates over the underlying data, keeping the lifetime in sync
    data_iter: std::slice::Iter<'a, T>,
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let coords = self.coords_iter.next()?;
        let value = self.data_iter.next()?;
        Some((coords, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.data_iter.size_hint()
    }
}

/// An iterator over the (i, j) coordinates and mutable references (`&mut T`) of all elements.
pub(crate) struct GridMutIterator<'a, T> {
    width: usize,
    // Iterates over the coordinates
    coords_iter: GridCoordsIterator,
    // Iterates mutably over the underlying data, keeping the lifetime in sync
    data_iter: std::slice::IterMut<'a, T>,
}

impl<'a, T> Iterator for GridMutIterator<'a, T> {
    type Item = ((usize, usize), &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let coords = self.coords_iter.next()?;
        let value = self.data_iter.next()?;
        Some((coords, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.data_iter.size_hint()
    }
}

impl<T: Default + Copy> Grid<T> {
    /// Creates a new grid initialized with `T::default()`.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }

    /// Creates a new grid initialized with `elem`.
    pub fn new_with(width: usize, height: usize, elem: T) -> Self {
        Self {
            width,
            height,
            data: vec![elem; width * height],
        }
    }

    pub fn is_empty(&self) -> bool {
        (self.width() * self.height()) == 0
    }

    pub fn row(&self, r: usize) -> &[T] {
        &self.data[r * self.width..(r + 1) * self.width]
    }

    pub fn row_mut(&mut self, r: usize) -> &mut [T] {
        &mut self.data[r * self.width..(r + 1) * self.width]
    }

    /// Returns an iterator over all (i, j) coordinates (column, row) in the grid.
    pub fn coords_iter(&self) -> GridCoordsIterator {
        GridCoordsIterator {
            width: self.width,
            height: self.height,
            current_i: 0,
            current_j: 0,
        }
    }

    /// Returns an iterator over the (i, j) coordinates and references (&T) of all elements.
    /// Elements are iterated in row-major order: (0, 0), (1, 0), ..., (width-1, height-1).
    pub fn iter(&self) -> GridIterator<'_, T> {
        GridIterator {
            width: self.width,
            coords_iter: self.coords_iter(),
            data_iter: self.data.iter(),
        }
    }

    /// Returns a mutable iterator over the (i, j) coordinates and mutable references (&mut T) of all elements.
    /// Elements are iterated in row-major order: (0, 0), (1, 0), ..., (width-1, height-1).
    pub fn iter_mut(&mut self) -> GridMutIterator<'_, T> {
        GridMutIterator {
            width: self.width,
            coords_iter: self.coords_iter(),
            data_iter: self.data.iter_mut(),
        }
    }

    /// Converts (column, row) coordinates into a 1D vector offset.
    /// Panics if coordinates are out of bounds.
    #[inline]
    fn idx_to_offset(&self, i: usize, j: usize) -> usize {
        assert!(
            i < self.width,
            "Column index out of bounds: {} >= {}",
            i,
            self.width
        );
        assert!(
            j < self.height,
            "Row index out of bounds: {} >= {}",
            j,
            self.height
        );
        self.width * j + i
    }

    /// Calculates the next coordinates when moving by a delta, if within bounds.
    /// Returns `Some((new_i, new_j))` or `None` if the move is out of bounds.
    #[inline]
    pub fn next_coords(&self, i: isize, j: isize, di: isize, dj: isize) -> Option<(usize, usize)> {
        let new_i = i.checked_add(di);
        let new_j = j.checked_add(dj);

        match (new_i, new_j) {
            (Some(ni), Some(nj)) if self.is_within_bounds(ni, nj) => {
                Some((ni as usize, nj as usize))
            }
            _ => None,
        }
    }

    /// Returns a reference to the element at `(i, j)` if within bounds.
    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        if i < self.width && j < self.height {
            // Safety: We've checked the bounds, so the index is valid.
            Some(unsafe { self.data.get_unchecked(self.width * j + i) })
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at `(i, j)` if within bounds.
    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        if i < self.width && j < self.height {
            // Safety: We've checked the bounds, so the index is valid.
            Some(unsafe { self.data.get_unchecked_mut(self.width * j + i) })
        } else {
            None
        }
    }

    /// Checks if the given (isize, isize) coordinates are within the grid boundaries.
    pub fn is_within_bounds(&self, i: isize, j: isize) -> bool {
        i >= 0 && j >= 0 && (i as usize) < self.width && (j as usize) < self.height
    }

    /// Returns an iterator over the (i, j) coordinates and references of the 8 neighbors
    /// (including diagonals) for a given coordinate (i, j).
    pub fn neighbors_8(
        &self,
        i: usize,
        j: usize,
    ) -> impl Iterator<Item = ((usize, usize), &T)> + '_ {
        DIRECTIONS_8.iter().filter_map(move |&(di, dj)| {
            let current_i = i as isize;
            let current_j = j as isize;

            self.next_coords(current_i, current_j, di, dj)
                .and_then(|(ni, nj)| self.get(ni, nj).map(|val| ((ni, nj), val)))
        })
    }

    /// Returns an iterator over the (i, j) coordinates and references of the 4 neighbors
    /// (excluding diagonals) for a given coordinate (i, j).
    pub fn neighbors_4(
        &self,
        i: usize,
        j: usize,
    ) -> impl Iterator<Item = ((usize, usize), &T)> + '_ {
        DIRECTIONS_4.iter().filter_map(move |&(di, dj)| {
            let current_i = i as isize;
            let current_j = j as isize;

            self.next_coords(current_i, current_j, di, dj)
                .and_then(|(ni, nj)| self.get(ni, nj).map(|val| ((ni, nj), val)))
        })
    }

    /// Creates a grid from multiline string input, mapping each character using `mapper`.
    /// This version is optimized for reading the whole input and allocating once.
    pub fn from_input_with<F>(input: &str, mapper: F) -> Result<Self>
    where
        F: Fn(char) -> T,
        T: Default,
    {
        let lines: Vec<&str> = input
            .lines()
            .map(str::trim_end)
            .filter(|l| !l.is_empty())
            .collect();

        if lines.is_empty() {
            bail!("No non-empty input lines to form grid");
        }

        let width = lines[0].len();
        if width == 0 {
            bail!("Input lines must not be empty");
        }

        for (line_num, line) in lines.iter().enumerate() {
            if line.len() != width {
                bail!(
                    "Inconsistent row width at line {}: expected {} but got {}",
                    line_num + 1,
                    width,
                    line.len()
                );
            }
        }

        let height = lines.len();
        let capacity = width * height;
        let mut data = Vec::with_capacity(capacity);

        for line in lines {
            data.extend(line.chars().map(&mapper));
        }

        // Final check: if `data.extend` failed to match capacity, something went wrong with the length calculation.
        if data.len() != capacity {
            bail!(
                "Internal error: Calculated capacity {} but generated {} elements.",
                capacity,
                data.len()
            );
        }

        Ok(Self {
            width,
            height,
            data,
        })
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }
}

// Custom implementation for Grid<char> for convenience
impl Grid<char> {
    pub fn from_input(input: &str) -> Result<Self> {
        Self::from_input_with(input, |c| c)
    }
}

// Indexing allows for `grid[(i, j)]` access
impl<T: Default + Copy> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        &self.data[self.idx_to_offset(i, j)]
    }
}

// Mutable indexing allows for `grid[(i, j)] = value;`
impl<T: Default + Copy> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (i, j) = index;
        let offset = self.idx_to_offset(i, j);
        &mut self.data[offset]
    }
}

// Display implementation for easy printing of the grid
impl<T: Default + Copy + std::fmt::Display> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in 0..self.height {
            for i in 0..self.width {
                let value = self.data[self.width * j + i];
                write!(f, "{}", value)?;
            }
            if j < self.height - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl<T: Default + Copy> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coords_iterator() {
        let grid = Grid::<i32>::new(3, 2); // 3 columns (i=0,1,2), 2 rows (j=0,1)

        let expected_coords = vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)];

        let actual_coords: Vec<(usize, usize)> = grid.coords_iter().collect();

        assert_eq!(actual_coords, expected_coords);
        assert_eq!(grid.coords_iter().len(), 6);
    }

    #[test]
    fn test_iter() {
        let mut grid = Grid::new(2, 2);
        grid[(0, 0)] = 1;
        grid[(1, 0)] = 2;
        grid[(0, 1)] = 3;
        grid[(1, 1)] = 4;

        let expected = vec![((0, 0), 1), ((1, 0), 2), ((0, 1), 3), ((1, 1), 4)];

        let actual: Vec<((usize, usize), i32)> =
            grid.iter().map(|(coords, &val)| (coords, val)).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_iter_mut() {
        let mut grid = Grid::new(2, 1); // 2x1 grid
        grid[(0, 0)] = 10;
        grid[(1, 0)] = 20;

        for ((i, _j), val) in grid.iter_mut() {
            // Double the value based on its column index
            if i == 0 {
                *val *= 2; // 20
            } else {
                *val += 10; // 30
            }
        }

        assert_eq!(grid[(0, 0)], 20);
        assert_eq!(grid[(1, 0)], 30);
    }
}
