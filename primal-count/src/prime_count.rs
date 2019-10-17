use crate::util::{integer_square_root, integer_cubic_root, integer_quartic_root};
use primal_sieve::Sieve;
use std::collections::HashMap;

fn create_prime_array(bound: usize) -> Vec<usize> {
    // Returns an array of primes up to and including bound
    let mut vec = Vec::new();
    let max_value_to_check = integer_square_root(bound);
    let mut sieve = vec![true; bound + 1];
    sieve[0] = false;
    sieve[1] = false;
    for value in 2..=bound {
        if sieve[value] {
            // Add the prime to our vector
            vec.push(value);

            // Filter primes more if we're below the bound
            if value <= max_value_to_check {
                // Anything smaller than this already filtered
                let mut idx = value * value;
                while idx <= bound {
                    sieve[idx] = false;
                    idx += value;
                }
            }
        }
    }
    return vec;
}

fn meissel_fn_small(m: usize, n: usize, primes: &Sieve, meissel_cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if n == 0 {
        return m;
    }
    match meissel_cache.get(&(m, n)).map(|entry| entry.clone()){
        Some(result) => result,
        None => {
            let value = meissel_fn_small(m, n-1, &primes, meissel_cache) 
                        - meissel_fn_small(m / primes.nth_prime(n), n-1, &primes, meissel_cache);
            meissel_cache.insert((m, n), value);
            return value;
        }
    }
}

pub fn meissel_fn(m: usize, n: usize, primes: &Sieve, meissel_cache: &mut HashMap<(usize, usize), usize>) -> usize {
    meissel_fn_small(m, n, &primes, meissel_cache)
}

fn num_primes_less_than_memoized(bound: usize, primes: &Sieve, prime_cache: &mut HashMap<usize, usize>, meissel_cache: &mut HashMap<(usize, usize), usize>) -> usize {
    // Initialise prime array:
    match prime_cache.get(&bound).map(|entry| entry.clone()){
        Some(value) => value,
        None => { // The meat of the function
            if bound < 2 {
                return 0;
            } else if bound < primes.upper_bound() {
                let result = primes.prime_pi(bound);
                prime_cache.insert(bound, result);
                return result;
            }
            
            let sqrt_bound = integer_square_root(bound);

            let nprimes_below_4thr = num_primes_less_than_memoized(integer_quartic_root(bound), primes, prime_cache, meissel_cache);
            let nprimes_below_3rdr = num_primes_less_than_memoized(integer_cubic_root(bound), primes, prime_cache, meissel_cache);
            let nprimes_below_2ndr = num_primes_less_than_memoized(sqrt_bound, primes, prime_cache, meissel_cache);

            // Issues with underflow here if nprimes_below_2ndr + nprimes_below_4thr < 2
            let mut result = ((nprimes_below_2ndr + nprimes_below_4thr - 2) * (nprimes_below_2ndr - nprimes_below_4thr + 1)) / 2;
            result += meissel_fn(bound, nprimes_below_4thr, &primes, meissel_cache);

            for i in nprimes_below_4thr..nprimes_below_2ndr {
                let ith_prime = primes.nth_prime(i+1);
                result -= num_primes_less_than_memoized(bound / ith_prime, primes, prime_cache, meissel_cache);
                if i < nprimes_below_3rdr {
                    let bi = num_primes_less_than_memoized(integer_square_root(bound / ith_prime), primes, prime_cache, meissel_cache);
                    for j in i..bi {
                        let jth_prime = primes.nth_prime(j+1);
                        result -= num_primes_less_than_memoized(bound / ith_prime / jth_prime, primes, prime_cache, meissel_cache) - j;
                    }
                }
            }

            // Caching and shit
            prime_cache.insert(bound, result);
            return result;
        }
    }
}

// Top level function
pub fn primes_below(bound: usize) -> usize {
    //let primes = create_prime_array(integer_square_root(bound));
    let primes = Sieve::new(integer_square_root(bound));
    let mut value_cache = HashMap::new();

    // Insert some basic primes - this is mainly to deal with underflow issues later
    value_cache.insert(0, 0);
    value_cache.insert(1, 0);
    value_cache.insert(2, 1);
    value_cache.insert(3, 2);
    value_cache.insert(4, 2);
    value_cache.insert(5, 3);
    value_cache.insert(6, 3);
    value_cache.insert(7, 4);
    value_cache.insert(8, 4);
    let mut meissel_cache = HashMap::new();
    let value = num_primes_less_than_memoized(bound, &primes, &mut value_cache, &mut meissel_cache);
    return value;
}
