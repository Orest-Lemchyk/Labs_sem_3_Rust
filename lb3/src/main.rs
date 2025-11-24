fn main() {

    // Матриця A
    let a = vec![
        vec![-1.0, 0.0, 1.0, 1.0],
        vec![2.0, 1.0, -1.0, 3.0],
        vec![-1.0, 0.0, 3.0, 2.0],
        vec![1.0, 2.0, 2.0, 1.0],
    ];

    // Матриця D
    let d = vec![
        vec![1.0, 1.0, 0.0, -1.0],
        vec![-2.0, 1.0, -2.0, 0.0],
        vec![-1.0, 0.0, 1.0, 2.0],
        vec![-1.0, 2.0, 3.0, 1.0],
    ];

    // Матриця F
    let f = vec![
        vec![-1.0, 1.0, 0.0, -1.0],
        vec![2.0, 1.0, -2.0, 0.0],
        vec![-1.0, 0.0, 0.0, 2.0],
        vec![1.0, -2.0, 4.0, 1.0],
    ];

    // Вектор y
    let y = vec![2.0, 5.0, -1.0, -2.0];

    // Вектор t
    let t = vec![1.0, 2.0, 0.0, 5.0];

    println!("--- Розрахунок Варіанту 11: (2F + 3AD)(2y + 5t) ---");

    let ad = multiply_matrix_matrix(&a, &d);
    
    let three_ad = multiply_scalar_matrix(3.0, &ad);

    let two_f = multiply_scalar_matrix(2.0, &f);

    let left_matrix = add_matrices(&two_f, &three_ad); 
    
    println!("Матриця лівої частини (2F + 3AD):");
    print_matrix(&left_matrix);

    let two_y = multiply_scalar_vector(2.0, &y);

    let five_t = multiply_scalar_vector(5.0, &t);

    let right_vector = add_vectors(&two_y, &five_t);
    
    println!("\nВектор правої частини (2y + 5t):");
    print_vector(&right_vector);

    let final_result = multiply_matrix_vector(&left_matrix, &right_vector);

    println!("\n=== Фінальний результат ===");
    print_vector(&final_result);
}


fn multiply_matrix_matrix(matrix_a: &Vec<Vec<f64>>, matrix_b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let rows_a = matrix_a.len();
    let cols_a = matrix_a[0].len();
    let rows_b = matrix_b.len();
    let cols_b = matrix_b[0].len();

    if cols_a != rows_b {
        panic!("Розміри матриць несумісні!");
    }

    let mut result = vec![vec![0.0; cols_b]; rows_a];

    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                result[i][j] += matrix_a[i][k] * matrix_b[k][j];
            }
        }
    }
    result
}

fn multiply_matrix_vector(matrix: &Vec<Vec<f64>>, vector: &Vec<f64>) -> Vec<f64> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    if cols != vector.len() {
        panic!("Розміри несумісні!");
    }

    let mut result = vec![0.0; rows];

    for i in 0..rows {
        for j in 0..cols {
            result[i] += matrix[i][j] * vector[j];
        }
    }
    result
}

fn multiply_scalar_matrix(scalar: f64, matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut result = vec![vec![0.0; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            result[i][j] = matrix[i][j] * scalar;
        }
    }
    result
}

fn multiply_scalar_vector(scalar: f64, vector: &Vec<f64>) -> Vec<f64> {
    let mut result = vec![0.0; vector.len()];
    for i in 0..vector.len() {
        result[i] = vector[i] * scalar;
    }
    result
}

fn add_matrices(matrix_a: &Vec<Vec<f64>>, matrix_b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let rows = matrix_a.len();
    let cols = matrix_a[0].len();
    
    if rows != matrix_b.len() || cols != matrix_b[0].len() {
        panic!("Розміри повинні співпадати!");
    }

    let mut result = vec![vec![0.0; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            result[i][j] = matrix_a[i][j] + matrix_b[i][j];
        }
    }
    result
}

fn add_vectors(vec_a: &Vec<f64>, vec_b: &Vec<f64>) -> Vec<f64> {
    if vec_a.len() != vec_b.len() {
        panic!("Розміри векторів повинні співпадати!");
    }

    let mut result = vec![0.0; vec_a.len()];
    for i in 0..vec_a.len() {
        result[i] = vec_a[i] + vec_b[i];
    }
    result
}

fn print_matrix(matrix: &Vec<Vec<f64>>) {
    for row in matrix {
        println!("{:?}", row);
    }
}

fn print_vector(vector: &Vec<f64>) {
    println!("{:?}", vector);
}