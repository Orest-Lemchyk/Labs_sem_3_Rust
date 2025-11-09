fn sum_squares_between(arr: &[f64]) -> Result<f64, &'static str> {
    if arr.len() < 2 {
        return Err("Масив занадто малий");
    }


    let mut negatives: Vec<(f64, usize)> = arr.iter()
    .enumerate()
    .filter(|&(_, &v)| v < 0.0)
    .map(|(i, &v)| (v, i))
    .collect();


    if negatives.len() < 2 {
        return Err("У масиві менше двох від'ємних елементів");
    }

    negatives.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    let second_largest_negative_index = negatives[1].1;

    let mut min_pos_value: Option<f64> = None;
    let mut min_pos_index: Option<usize> = None;
    for (i, &v) in arr.iter().enumerate() {
        if v > 0.0 {
            match min_pos_value {
                None => {
                    min_pos_value = Some(v);
                    min_pos_index = Some(i);
                }
                Some(cur_min) => {
                    if v < cur_min {
                        min_pos_value = Some(v);
                        min_pos_index = Some(i);
                    }
                }
            }
        }
    }

    let min_pos_index = match min_pos_index {
        Some(i) => i,
        None => return Err("У масиві немає додатних елементів"),
    };

    let i1 = second_largest_negative_index;
    let i2 = min_pos_index;
    if (i1 as isize - i2 as isize).abs() <= 1 {
        return Ok(0.0);
    }

    let (start, end) = if i1 < i2 { (i1 + 1, i2) } else { (i2 + 1, i1) };

    let mut sum = 0.0;
    for v in &arr[start..end] {
        sum += v * v;
    }
    Ok(sum)
}

fn main() {
    let arr: [f64; 36] = [
        0.5, -0.2, 1.0, -1.5, 2.3, -0.9, 0.1, 3.0, -0.05, 0.05,
        -2.0, 1.5, 0.5, -0.8, 2.0, -4.0, 0.001, 0.7, -0.3, 0.2,
        0.2, -0.6, 1.1, -1.1, 0.9, 0.01, -0.01, 2.5, -3.3, 0.4,
        -0.4, 0.6, 1.7, -0.2, 0.8, -0.7
    ];

    match sum_squares_between(&arr) {
        Ok(sum) => println!("Сума квадратів між другим найбільшим від'ємним та найменшим додатним: {}", sum),
        Err(msg) => println!("Не вдалося обчислити: {}", msg),
    }
}
