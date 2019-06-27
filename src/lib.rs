#[derive(Debug)]
pub struct PrimeSieve(pub Vec<u32>);

impl PrimeSieve {
    pub fn new(below: u32) -> Self {
        let mut sieve = vec![true; below as usize];
        let mut primes = Vec::new();

        sieve.splice(0..2, [false; 2].iter().cloned());

        for idx in 0..sieve.len() {
            let candidate = sieve[idx];
            if candidate == true {
                let mut iter = sieve.iter_mut();
                if iter.nth((idx * idx) - 1).is_some() {
                    for b in iter.step_by(idx) {
                        *b = false;
                    }
                }
                primes.push(idx as u32);
            }
        }

        PrimeSieve(primes)
    }
}

#[derive(Debug)]
pub struct PrimeGen {
    below: Option<u32>,
    counter: Option<u32>,
    primes: Vec<u32>,
}

impl PrimeGen {
    pub fn new(below: Option<u32>) -> Self {
        Self {
            below,
            counter: None,
            primes: Vec::new(),
        }
    }
}

impl Iterator for PrimeGen {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let next_prime = if let Some(counter) = self.counter {
            let mut candidate = counter + 2;
            while self
                .primes
                .iter()
                .filter(|&x| *x <= ((candidate as f32).sqrt() as u32))
                .any(|&x| candidate % x == 0)
            {
                candidate += 2
            }
            self.counter = Some(candidate);
            self.primes.push(candidate);
            candidate
        } else {
            self.counter = Some(1);
            self.primes.push(2);
            2
        };
        match self.below {
            Some(num) if num <= next_prime => None,
            _ => Some(next_prime),
        }
    }
}
