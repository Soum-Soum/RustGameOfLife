use rand::random_bool;

use bevy::ecs::{component::Component, system::Resource};

#[derive(Component, Clone, Copy)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

#[derive(Resource)]
pub struct Grid {
    pub height: usize,
    pub width: usize,
    pub cells: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            cells: vec![vec![false; width as usize]; height as usize],
        }
    }

    fn is_coords_valid(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        return true;
    }

    fn assert_coords_valid(&self, x: usize, y: usize) {
        assert!(
            self.is_coords_valid(x, y),
            "Invalid coordinates : x = {}, y = {} (grid: {}x{})",
            x,
            y,
            self.width,
            self.height
        );
    }

    pub fn get_value(&self, x: usize, y: usize) -> bool {
        self.assert_coords_valid(x, y);
        return self.cells[y][x];
    }

    fn set_value(&mut self, x: usize, y: usize, value: bool) -> () {
        self.assert_coords_valid(x, y);
        self.cells[y][x] = value
    }

    pub fn toggle_value(&mut self, x: usize, y: usize) -> () {
        self.set_value(x, y, !self.get_value(x, y));
    }

    fn count_alive_neighbors(&self, x: usize, y: usize) -> i32 {
        let xi = x as i32;
        let yi = y as i32;

        let mut alive_neighbors: i32 = 0;
        for i in xi - 1..=xi + 1 {
            for j in yi - 1..=yi + 1 {
                if j == yi && i == xi {
                    continue; // ignore the central cell
                }
                if !self.is_coords_valid(i as usize, j as usize) {
                    continue; // ignore out of bounds
                }
                if self.get_value(i as usize, j as usize) {
                    alive_neighbors += 1
                }
            }
        }
        return alive_neighbors;
    }

    pub fn update(&mut self) -> () {
        let mut new_cells = vec![vec![false; self.width]; self.height];

        for x in 0..(self.width - 1) {
            for y in 0..(self.height - 1) {
                let alive_neighbors = self.count_alive_neighbors(x, y);
                let current_cell_is_alive = self.get_value(x, y);

                if current_cell_is_alive {
                    if alive_neighbors == 2 || alive_neighbors == 3 {
                        new_cells[y][x] = true;
                    }
                } else {
                    if alive_neighbors == 3 {
                        new_cells[y][x] = true;
                    }
                }
            }
        }
        self.cells = new_cells
    }

    pub fn reset_random(&mut self, p: f64) -> () {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_value(x, y, random_bool(p));
            }
        }
    }

    pub fn clear(&mut self) -> () {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_value(x, y, false);
            }
        }
    }
}

pub struct GridIterator<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = (usize, usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let is_end = self.y >= self.grid.height;
        if is_end {
            return None;
        }

        let x_cpy = self.x;
        let y_cpy = self.y;
        let value = self.grid.get_value(self.x, self.y);

        self.x += 1;
        if self.x >= self.grid.width {
            self.x = 0;
            self.y += 1;
        }

        return Some((x_cpy, y_cpy, value));
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = (usize, usize, bool);

    type IntoIter = GridIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GridIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}
