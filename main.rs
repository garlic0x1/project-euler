struct Queue {
    queue: Vec<u8>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            queue: Vec::new()
        }
    }

    fn enqueue(&mut self, item: u8) {
        self.queue.push(item);
    }
    
    fn dequeue(&mut self) -> u8 {
        self.queue.remove(0)
    }
    
    fn length(&self) -> usize {
        self.queue.len()
    }

    fn product(&self) -> u64 {
        let mut prod: u64 = 1;
        for i in self.queue.iter() {
            let n = *i as u64;

            prod = prod * n;
        }
        
        return prod;
    }
}

fn main() {
    let euler8 = String::from("7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450");

    let mut queue: Queue = Queue::new();
    let mut max = 0;
    
    for c in euler8.chars() {
        let n = c.to_digit(10).unwrap() as u8;
        queue.enqueue(n);
        if queue.length() > 13 {
            queue.dequeue();
        }
        let prod = queue.product();
        if prod > max {
            max = prod;
        }
    }
    println!("{}", max);
}
