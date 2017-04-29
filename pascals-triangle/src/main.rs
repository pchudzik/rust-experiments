fn main() {
    let triangle = build_triangle(10);
    for row in triangle {
    	println!("{:?}", row);
    }
}

fn build_triangle(rows: i32) -> Vec<Vec<i32>> {
	let mut result:Vec<Vec<i32>> = vec![];
	while result.len() <= rows as usize {
		let previous_row = result.pop().unwrap_or(vec![]);
		let next_row = build_row(&previous_row);
		result.push(previous_row);
		result.push(next_row);
	}
	result.remove(0);
	return result;
}

fn build_row(previous_row: &Vec<i32>) -> Vec<i32> {
	if previous_row.is_empty() {
		vec![1]
	} else if previous_row.len() == 1 {
		vec![1,1]
	} else {
		let mut result = vec![1];
		for index in 0..previous_row.len()-1 {
			result.push(previous_row[index] + previous_row[index+1]);
		}
		result.push(1);
		return result;
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn should_build_triangle_row() {
		//http://mathforum.org/workshops/usi/pascal/images/row8sum.gif
		assert_eq!(
			build_row(&vec![1]), 
			vec![1,1]);

		assert_eq!(
			build_row(&vec![1,1]), 
			vec![1, 2, 1]);

		assert_eq!(
			build_row(&vec![1, 3, 3, 1]),
			vec![1, 4, 6, 4, 1]);

		assert_eq!(
			build_row(&vec![1, 8, 28, 56, 70, 56, 28, 8, 1]),
			vec!(1, 9, 36, 84, 126, 126, 84, 36, 9, 1));
	}
}
