fn main() {
    for a in 0..333 {
        for b in 0..500 {
            for c in 333..1000 {
                if is_triple(a, b, c) && a + b + c == 1000 {
                    println!("{}",a*b*c);
                    return;
                }
            }
        }
    }
}

fn is_triple(a: i32, b: i32, c: i32) -> bool {
    (a * a) + (b * b) == (c * c)
}
