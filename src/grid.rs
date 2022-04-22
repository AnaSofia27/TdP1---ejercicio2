use crate::file::File;
use std::cmp;

///Representacion de una Grilla.
pub struct Grid {
    grid: Vec<Vec<i32>>,
}

impl Grid {
    /// Dado un numero de columnas(m) y un numero de filas(n), devuelve una grilla(mxn) inizializada con ceros.
    pub fn new(n_columns: usize, n_rows: usize) -> Self {
        let grid = vec![vec![0; n_columns]; n_rows];
        Grid { grid }
    }

    /// Dado una posicion en la grilla (m y n) y un valor, ubica dicho valor en la posicion indicada en la grilla.
    pub fn write(&mut self, row: usize, column: usize, value: i32) {
        self.grid[row][column] = value;
    }

    /// Dada una posicion de la grilla (m y n) devuelve el valor en esa posicion.
    pub fn read(&self, row: usize, column: usize) -> &i32 {
        &self.grid[row][column]
    }
    /// Dado dos archivos, devuelve la grilla del algoritmo LCS.
    pub fn longest_common_subsequence_grid(&mut self, file1: &File, file2: &File) {
        for (i, _i) in file1.get_vec_lines().iter().enumerate() {
            for (j, _j) in file2.get_vec_lines().iter().enumerate() {
                if file1.get_vec_lines()[i] == file2.get_vec_lines()[j] {
                    self.write(i + 1, j + 1, self.read(i, j) + 1);
                } else {
                    self.write(
                        i + 1,
                        j + 1,
                        cmp::max(*self.read(i + 1, j), *self.read(i, j + 1)),
                    );
                }
            }
        }
    }
}
