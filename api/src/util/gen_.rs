use rand::random_range;

/// Generates random alphanumeric code
pub fn generate_random_code(length: usize) -> String {
    if length < 1 {
        panic!("length should be > 0")
    }
    let alphanum: Vec<char> = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9',
    ];
    let mut random = String::new();
    while random.chars().count() <= length {
        random.push(alphanum[random_range(0..alphanum.len())]);
    }
    random
}
