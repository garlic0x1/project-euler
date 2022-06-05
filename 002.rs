fn main() {
    let mut sum = 2;
    let mut reg1 = 1;
    let mut reg2 = 2;

    loop {
        let newval = reg1 + reg2;
        if newval > 4000000 {
            break;
        } else if newval % 2 == 0 {
            sum += newval;
        }
        reg1 = reg2;
        reg2 = newval;
    }

    println!("{}", sum);
}
