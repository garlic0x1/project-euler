use math::bignums::big_integer::BigInteger;

pub fn pe65() {
    let mut num = BigInteger::from_u32(2, None);
    let mut last = BigInteger::from_u32(1, None);

    let mut terms = (2..).map(|n| if n % 3 == 0 { 2 * (n / 3) } else { 1 });
    let mut c = 0;

    while let Some(term) = terms.next() {
        if c == 99 {
            break;
        }

        let temp = last;
        last = num.clone();
        num.multiply(&BigInteger::from_u32(term, None));
        num.add(&temp);

        c += 1;
    }

    println!("{}", num.raw_vec().iter().fold(0u64, |s, n| s + *n as u64));
}
