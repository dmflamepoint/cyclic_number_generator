// David Miller
// March 24th, 2021
//
// This program will find large cyclic numbers by taking advantage of the relationship between
// cyclic numbers and primitve roots module p. To do so, there needs to be a relatively fast
// algorithm for finding the prime factorization of a number x.


// Primes is a structure that stores the current list of primes needed for the euler_phi function
pub struct Primes(Vec<u32>);

impl Primes {
    pub fn add_primes(&mut self, x: u32) -> &Vec<u32> {
        //println!("Array before adding to {} is \n{:?}\n", x, self.0);

        let length: usize = self.0.len();
        let mut largest_prime_candidate: u32 = 0;
        if length >= 2 {
            largest_prime_candidate = self.0[length-1];
        } else {
            // push 2 and 3 so the largest prime is odd
            self.0.push(2u32);
            self.0.push(3u32);
            largest_prime_candidate = 3; 
        }
        
        // counting by 2's and adding prime numbers found until x is reached
        let mut is_prime: bool = true;
        while largest_prime_candidate < x {
            largest_prime_candidate += 2;
            for prime in &self.0 {
                if largest_prime_candidate % prime == 0 {
                    is_prime = false;
                    break;
                }
            }
            
            if is_prime {
                self.0.push(largest_prime_candidate);
            } else {
                is_prime = true;
            }

        }

        return &self.0;
    
    }
}


// euler_phi(x) returns the number of positive integers not exceeding x that are coprime to x
fn euler_phi(list_of_primes: &mut Primes, x: u32) -> u32 {
    let mut result: u32 = 1;
    
    let factors: Vec<(u32, u32)> = prime_factorization(list_of_primes, x);
    for pair in factors {
        result *= pair.0.pow(pair.1-1)*(pair.0-1);
    }
    
    return result;
}


// prime_factorization(x) returns the prime factorization of an integer x stored as a vector of
// tuples. 60 = 2^2 * 3 * 5 would be returned as [(2,2), (3,1), (5,1)].
fn prime_factorization(list_of_primes: &mut Primes, x: u32) -> Vec<(u32,u32)> {
    let mut factors: Vec<(u32, u32)> = Vec::new();
    let mut remainder: u32 = x;
    let primes: Vec<u32> = list_of_primes.add_primes(x).to_vec();
        
    for prime in primes {
        // remainder should never be less than 1, but if it is, there are no more prime factors
        // left to find. Equality to 1 is the actual stopping condition
        if remainder <= 1 {
            break;
        }

        let mut power: u32 = 0;
        while remainder % prime == 0 {
            remainder /= prime;
            power += 1;
        }
        if power != 0 {
            factors.push((prime, power));
        }
    }
    return factors;
}


fn main() {
    let mut my_vec: Vec<u32> = Vec::new();
    let mut list_of_primes = Primes(my_vec);
    
    let _a = list_of_primes.add_primes(1000);
    let _b = list_of_primes.add_primes(10000);
    let _c = list_of_primes.add_primes(100000);

    //let mut test_value: u32 = 50;
    //let mut result = euler_phi(&mut list_of_primes, test_value);
    
    //println!("phi({}) = {}", test_value, result);
    
    //println!("\n\n");

    //test_value = 501;
    //result = euler_phi(&mut list_of_primes, test_value);
    
    //println!("phi({}) = {}", test_value, result);
    
    //let p: Vec<u32> = list_of_primes.add_primes(100).to_vec();
    //println!("{:?}", p);

    //let q: Vec<u32> = list_of_primes.add_primes(1000).to_vec();
    //println!("{:?}", q);

}



