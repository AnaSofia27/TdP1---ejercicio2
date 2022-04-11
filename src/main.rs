use std::cmp;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename1 = &args[1];
    let filename2 = &args[2];

    let file1 = read_file_lines(filename1);
    let file2 = read_file_lines(filename2);

    let filename1_size = file1.len();
    let filename2_size = file2.len();

    let grid = longest_common_subsequence(&file1, &file2);
    diff(&grid, &file1, &file2, filename1_size, filename2_size);
}

/// Recibe el path de un archivo y devuelve un vector cuyos elementos son cada linea del archivo.
fn read_file_lines(filepath: &str) -> Vec<String> {
    let file_content = fs::read_to_string(filepath).expect("Error: Unable to read the file");
    let split_lines = file_content.split('\n');

    split_lines.map(str::to_string).collect()
}

/// A partir de un numero de columnas (m) y un numero de filas (n) dado, devuelve una grilla (mxn) inizializada toda en cero.
fn create_grid(n_columns: usize, n_rows: usize) -> Vec<Vec<i32>> {
    vec![vec![0; n_columns]; n_rows]
}

/// Dado dos archivos devuelve una grilla del algoritmo para hallar la subsecuencia mas larga. 
fn longest_common_subsequence(file1: &[String], file2: &[String]) -> Vec<Vec<i32>> {
    let n_rows = file1.len() + 1;
    let n_columns = file2.len() + 1;
    let mut grid = create_grid(n_columns, n_rows);

    for (i, _i) in file1.iter().enumerate() {
        for (j, _j) in file2.iter().enumerate() {
            if file1[i] == file2[j] {
                grid[i + 1][j + 1] = grid[i][j] + 1;
            } else {
                grid[i + 1][j + 1] = cmp::max(grid[i + 1][j], grid[i][j + 1]);
            }
        }
    }
    grid
}

/// Dado dos archivos,su grila LCS y la longitud de cada archivo, imprime por pantalla la diferencia de lineas entre archivos.
fn diff(grid: &[Vec<i32>], file1: &[String], file2: &[String], m: usize, n: usize) {
    if m > 0 && n > 0 && file1[m - 1] == file2[n - 1] {
        diff(grid, file1, file2, m - 1, n - 1);
        println!("{}", file1[m - 1]);
    } else if n > 0 && (m == 0 || grid[m][n - 1] >= grid[m - 1][n]) {
        diff(grid, file1, file2, m, n - 1);
        println!("> {}", file2[n - 1]);
    } else if m > 0 && (n == 0 || grid[m][n - 1] < grid[m - 1][n]) {
        diff(grid, file1, file2, m - 1, n);
        println!("< {}", file1[m - 1]);
    } else {
        println!();
    }
}
