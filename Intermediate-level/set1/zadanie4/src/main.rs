fn main() {
    struct PrimeNumbers{
        current: u32,
        max: u32,
    }
    impl PrimeNumbers{
        fn new(max:u32)-> Self {
            PrimeNumbers {current:0 , max}
        }
    }

    impl Iterator for PrimeNumbers {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item>{
            while self.current < self.max {
                self.current += 1;
                if is_prime(self.current) {
                    return Some(self.current);
                }
            }
            None
        }
    }
    let primes = PrimeNumbers::new(17);
   
    for num in primes{
        println!("{}",num)
    }

    let sum:u32 = PrimeNumbers::new(17).sum();
    println!("{}",sum);

}


fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    let sqrt_num = (num as f64).sqrt() as u32;

    for n in 2..=sqrt_num {
        if num % n == 0 {
            return false; 
        }
    }

    true 
}