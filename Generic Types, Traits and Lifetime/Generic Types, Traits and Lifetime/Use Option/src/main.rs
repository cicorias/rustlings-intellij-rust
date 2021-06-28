fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(13);
    print_number(99);

    let mut numbers: [Option<u16>; 5]/* Something goes here */;
    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 1235) + 2) / (4 * 16)
        };

        numbers[iter as usize] = number_to_add;
    }
}
