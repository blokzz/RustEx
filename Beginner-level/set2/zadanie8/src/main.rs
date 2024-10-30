fn find_element(vec: Vec<i32>, value: i32) -> Option<usize> {
    for (index, &item) in vec.iter().enumerate() {
        if item == value {
            return Some(index);
        }
    }
    None
}

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    let value_to_find = 30;

    match find_element(numbers, value_to_find) {
        Some(index) => println!("Znaleziono wartość na pozycji: {}", index),
        None => println!("Nie znaleziono"),
    }
}
