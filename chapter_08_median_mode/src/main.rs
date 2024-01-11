use std::io;

fn main() {
    println!("Welcome to the Median & Mode solver!");

    let mut arr = read_vector_of_integers_from_user_input();

    println!("You've entered {:?}", arr);

    arr.sort_unstable();

    println!("Sorted array: {:?}", arr);

    println!("The median is {}", calculate_median_of_sorted_array(&arr));

    println!("The mode is {}", calculate_mode_of_sorted_array(&arr));
}

fn calculate_mode_of_sorted_array(arr: &Vec<usize>) -> usize {
    struct Freq {
        value: usize,
        quantity: usize
    }

    let mut max_freq = Freq{ value: 0, quantity: 0 };
    let mut current_freq = Freq{ value: 0, quantity: 0 };

    for val in arr {
        if *val == current_freq.value {
            current_freq.quantity += 1;
        } else {
            if current_freq.quantity > max_freq.quantity {
                max_freq = Freq{ .. current_freq };
            }

            current_freq = Freq{ quantity: 1, value: *val };
        }
    }

    if current_freq.quantity > max_freq.quantity {
        max_freq = Freq{ .. current_freq };
    }

    max_freq.value

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
