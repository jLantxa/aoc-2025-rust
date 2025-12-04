#![allow(dead_code)]

use anyhow::{Result, bail};

pub(crate) const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Clone)]
pub(crate) struct Grid<T: Default + Copy> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Default + Copy> Grid<T> {
    pub fn new_with_dimensions(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }

    pub fn new_with_width(width: usize) -> Self {
        Self {
            width,
            height: 1,
            data: vec![T::default(); width],
        }
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, i: usize, j: usize) -> &T {
        &self.data[self.width * j + i]
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.data[self.width * j + i]
    }

    pub fn extend_row(&mut self) {
        let current_size = self.width * self.height;
        self.data.resize(current_size + self.width, T::default());
        self.height += 1;
    }

    pub fn is_within_bounds(&self, i: isize, j: isize) -> bool {
        i >= 0 && j >= 0 && i < self.width as isize && j < self.height as isize
    }

    pub fn from_input_with<F>(input: &str, mapper: F) -> Result<Self>
    where
        F: Fn(char) -> T,
        T: Default,
    {
        let mut lines = input.lines();

        let first_line = match lines.next() {
            Some(line) => line,
            None => bail!("No input to form grid"),
        };

        let width = first_line.len();
        if width == 0 {
            bail!("Input lines must not be empty");
        }

        let mut grid = Self::new_with_width(width);

        for (i, ch) in first_line.chars().enumerate() {
            *grid.get_mut(i, 0) = mapper(ch);
        }

        for line in lines {
            let line = line.trim_end();

            if line.len() != width {
                bail!(
                    "Inconsistent row width: expected {} but got {}",
                    width,
                    line.len()
                );
            }

            grid.extend_row();
            let new_row_index = grid.height() - 1;

            for (i, ch) in line.chars().enumerate() {
                *grid.get_mut(i, new_row_index) = mapper(ch);
            }
        }

        Ok(grid)
    }
}

impl Grid<char> {
    pub fn from_input(input: &str) -> Result<Self> {
        let mut lines = input.lines();

        let first_line = match lines.next() {
            Some(line) => line,
            None => bail!("No input to form grid"),
        };

        let width = first_line.len();
        if width == 0 {
            bail!("Input lines must not be empty");
        }

        let mut grid = Self::new_with_width(width);

        for (i, ch) in first_line.chars().enumerate() {
            *grid.get_mut(i, 0) = ch;
        }

        for line in lines {
            let line = line.trim_end();

            if line.len() != width {
                bail!(
                    "Inconsistent row width: expected {} but got {}",
                    width,
                    line.len()
                );
            }

            grid.extend_row();
            let new_row_index = grid.height() - 1;

            for (i, ch) in line.chars().enumerate() {
                *grid.get_mut(i, new_row_index) = ch;
            }
        }

        Ok(grid)
    }
}

impl<T: Default + Copy + std::fmt::Display> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in 0..self.height {
            for i in 0..self.width {
                let value = self.get(i, j);

                write!(f, "{}", value)?;
            }

            if j < self.height - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
