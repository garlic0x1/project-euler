fn main() {
    println!("{}", sum_vec(&binary_vec(1000)));
}

fn sum_vec(arr: &Vec<u8>) -> usize {
    let mut sum: usize = 0;
    for i in arr.iter() {
        sum += *i as usize;
    }
    return sum;
}

fn binary_vec(power: usize) -> Vec<u8> {
    let mut arr: Vec<u8> = vec![1; 1];
    for _i in 0..power {
        let mut carry: u8 = 0;
        for n in 0..arr.len() {
            carry = double_place(&mut arr, n, carry);
        }
        if carry != 0 {
            arr.push(carry);
        }
    }
    return arr;
}

fn double_place(arr: &mut Vec<u8>, n: usize, carry: u8) -> u8 {
    let double = arr.get(n).unwrap() * 2;
    arr[n] = (double + carry) % 10;
    return double / 10;
}
