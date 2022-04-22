use crate::grid::Grid;
use std::fs;

///Representacion de un archivo con su contenido separado por lineas.
pub struct File {
    vec_lines: Vec<String>,
    n_lines: usize,
}
impl File {
    ///Constructor de File dado el path del archivo.
    pub fn new(filepath: &str) -> Self {
        let content = fs::read_to_string(filepath).expect("Error: Unable to read the file");
        let vec_lines: Vec<String> = content.split('\n').map(str::to_string).collect();
        let n_lines = vec_lines.len();

        File { vec_lines, n_lines }
    }
    /// Devuelve el archivo parseado por lineas en un vector.
    pub fn get_vec_lines(&self) -> &Vec<String> {
        &self.vec_lines
    }
    ///Dado dos archivos imprime la diferencia entre los mismos utilizando el algoritmo de LCS.
    pub fn diff(&self, file2: &File) {
        let mut grid = Grid::new(file2.n_lines + 1, self.n_lines + 1);
        grid.longest_common_subsequence_grid(self, file2);
        self._diff(&grid, file2, self.n_lines, file2.n_lines);
    }
    /// Funcion wrapper de diff.
    fn _diff(&self, grid: &Grid, file2: &File, m: usize, n: usize) {
        if m > 0 && n > 0 && self.vec_lines[m - 1] == file2.vec_lines[n - 1] {
            self._diff(grid, file2, m - 1, n - 1);
            println!("{}", self.vec_lines[m - 1]);
        } else if n > 0 && (m == 0 || grid.read(m, n - 1) >= grid.read(m - 1, n)) {
            self._diff(grid, file2, m, n - 1);
            println!("> {}", file2.vec_lines[n - 1]);
        } else if m > 0 && (n == 0 || grid.read(m, n - 1) < grid.read(m - 1, n)) {
            self._diff(grid, file2, m - 1, n);
            println!("< {}", self.vec_lines[m - 1]);
        } else {
            println!();
        }
    }
}
