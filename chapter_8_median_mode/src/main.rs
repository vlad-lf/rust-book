use std::io;

fn main() {
    println!("Welcome to the Median & Mode solver!");

    let mut arr = read_vector_of_integers_from_user_input();

    println!("You've entered {:?}", arr);

    arr.sort_unstable();

    println!("Sorted array: {:?}", arr);

    println!("The median is {}", calculate_median_of_sorted_array(&arr))
}

fn calculate_median_of_sorted_array(arr: &Vec<usize>) -> f64 {
    if arr.len() % 2 == 0 {
        (arr[arr.len() / 2] + arr[arr.len() / 2 - 1]) as f64 / 2.0
    } else {
        arr[arr.len() / 2] as f64
    }
}

fn read_vector_of_integers_from_user_input() -> Vec<usize> {
    'upper_loop: loop {
        println!("Please, enter the array of unsigned integers spliited by ','");

        let mut arr = String::new();

        io::stdin()
            .read_line(&mut arr)
            .expect("Failed to read line");

        let arr: Vec<&str> = arr.split(',').collect();

        let mut res: Vec<usize> = vec!();

        for number in arr {
            let casted_value: usize = match number.trim().parse() {
                Ok(number) => number,
                Err(_) => {
                    println!("Failed to cast '{number}' to unsigned integer, please make valid input");
                    continue 'upper_loop
                }
            };
            res.push(casted_value);
        } 

        return res;
    }
}
