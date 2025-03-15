fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn test_is_prime() {
    let test_data = [
        (10, false),
        (11, true),
        (14, false),
        (17, true),
        (18, false),
        (19, true),
        (21, false),
        (23, true),
    ];

    for (n, expected) in test_data.iter() {
        let result = is_prime(*n);
        if result == *expected {
            println!("{} passed the test", n);
        } else {
            println!("{} failed the test (expected: {}, got: {})", n, expected, result);
        }
    }
}

fn main() {
    test_is_prime();
}
