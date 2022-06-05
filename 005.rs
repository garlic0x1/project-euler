fn main() {
    let mut i = 20;
    let smallest_multiple = loop {
	if i%19 == 0 &&
	    i%18 == 0 &&
	    i%17 == 0 &&
	    i%16 == 0 &&
	    i%15 == 0 &&
	    i%14 == 0 &&
	    i%13 == 0 &&
	    i%11 == 0 &&
	    i%11 == 0 {
	    break i;
	}

        i += 20;
    };
    println!("{}", smallest_multiple);
}
