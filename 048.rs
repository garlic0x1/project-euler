/*


The series, 11 + 22 + 33 + ... + 1010 = 10405071317.

Find the last ten digits of the series, 11 + 22 + 33 + ... + 10001000.

*/

pub fn pe48() {
    let mut digits = [0; 10];

    for i in 1..=1000 {
        let pow = self_pow(i);
        println!("{i} ^ {i} = {}", num_str(&pow));
        add_nums(&mut digits, &pow);
    }

    println!("{}", num_str(&digits));
}

fn num_str(num: &[u32]) -> String {
    let mut s = String::new();
    for &d in num.iter().rev() {
        s.push_str(&format!("{d}"));
    }
    s
}

fn big_int(n: u32) -> [u32; 10] {
    let mut place: u64 = 1;
    let mut arr = [0; 10];
    for i in 0..10 {
        arr[i] = ((n as u64 % (place * 10)) / place) as u32;
        place *= 10;
    }
    arr
}

fn self_pow(n: u32) -> [u32; 10] {
    let mut digits = big_int(n); // 0
    for _ in 1..n {
        multiply_nums(&mut digits, n);
    }
    digits
}

fn multiply_nums(num: &mut [u32; 10], fact: u32) {
    let digits = num.clone();
    for _ in 1..fact {
        add_nums(num, &digits);
    }
}

fn add_nums(this: &mut [u32], that: &[u32]) {
    this.iter_mut().zip(that.iter()).fold(0, |carry, (is, at)| {
        let c = (carry + *at + *is) / 10;
        *is = (carry + *at + *is) % 10;
        c
    });
}
