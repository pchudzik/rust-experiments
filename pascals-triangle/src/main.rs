fn main() {
    let triangle = pascals_triangle(30);
    for row in triangle {
    	println!("{:?}", row);
    }
}

fn calculate_value(row: i32, index: i32) -> i32 {
	if index < 0 || index > row {
		0
	} else if index == 1 || index == row {
		1
	} else {
		calculate_value(row - 1, index -1) + calculate_value(row - 1, index)
	}
}

fn caluclate_row(row: i32) -> Vec<i32> {
	(1..row + 1)
		.map(|index| calculate_value(row, index))
		.collect::<Vec<i32>>()
}

fn pascals_triangle(rows:i32) -> Vec<Vec<i32>> {
	(1..rows + 1)
		.map(|row| caluclate_row(row))
		.collect::<Vec<Vec<i32>>>()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn should_build_pascals_triangle() {
		assert_eq!(
			pascals_triangle(8),
			vec![
				vec![1],
				vec![1, 1],
				vec![1, 2, 1],
				vec![1, 3, 3, 1],
				vec![1, 4, 6, 4, 1],
				vec![1, 5, 10, 10, 5, 1],
				vec![1, 6, 15, 20, 15, 6, 1],
				vec![1, 7, 21, 35, 35, 21, 7, 1]
			]);
	}
}
