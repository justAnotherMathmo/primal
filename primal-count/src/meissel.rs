extern crate primal_sieve;
use std::iter::FromIterator;

use crate::util::{int_square_root, int_cubic_root, int_quartic_root};
use std::collections::HashMap;

const MEISSEL_LOOKUP_SIZE: usize = 8;  // Number of primes we do the reduce trick for
const SMALL_PRIMES:         [usize; MEISSEL_LOOKUP_SIZE]     = [   2, 3,  5,   7,   11,    13,     17,      19];
const SMALL_PRIME_PRODUCTS: [usize; MEISSEL_LOOKUP_SIZE + 1] = [1, 2, 6, 30, 210, 2310, 30030, 510510, 9699690];
const SMALL_TOTIENT_VALUES: [usize; MEISSEL_LOOKUP_SIZE + 1] = [1, 1, 2,  8,  48,  480,  5760,  92160, 1658880];

/// Generate a vec of primes from 2 up to and including limit
/// Leverages the fast sieve in primal to do so
fn generate_primes(limit: usize) -> Vec<usize> {
    let sieve = primal_sieve::Sieve::new(limit);
    let sieve_iter = sieve.primes_from(2).take_while(|x| *x <= limit);
    // Note that we put the primes into a vec here because later we want to have both
    //  1) Very quick access to the nth prime
    //  2) Quick counting of number of primes below a value, achieved with a binary search
    // Experiments replacing 1) or 2) with the methods in sieve seem to significantly
    //   slow things down for larger numbers
    return Vec::from_iter(sieve_iter);
}

/// The number of numbers less than m that are coprime to the first n prime numbers
/// Get recurrence m_fn(m, n) = m_fn(m, n - 1) - m_fn(m/p_n, n-1) by thinking about p_n, the nth prime
fn small_meissel_fn(m: usize, n: usize, prime_array: &Vec<usize>, meissel_cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if n == 0 || m < 2 {
        return m;
    }
    if prime_array[n-1] >= m {
        return 1;
    }
    match meissel_cache.get(&(m, n)).map(|entry| entry.clone()){
        Some(result) => result,
        None => {
            // For small values of n, we can decrease the size of m by noting that
            // the meissel function is almost periodic with period p_1 * .. * p_n
            let mut value = 0;
            let mut m_shrunk = m;
            if n <= MEISSEL_LOOKUP_SIZE {
                value = (m / SMALL_PRIME_PRODUCTS[n]) * SMALL_TOTIENT_VALUES[n];
                m_shrunk = m_shrunk % SMALL_PRIME_PRODUCTS[n];
            }

            // After shrinkage, just apply the recursion
            value += small_meissel_fn(m_shrunk, n-1, &prime_array, meissel_cache) - small_meissel_fn(m_shrunk / prime_array[n-1], n-1, &prime_array, meissel_cache);
            meissel_cache.insert((m, n), value);
            return value;
        }
    }
}

const BETA: f32 = 1.0;
fn get_alpha(bound: usize) -> f32 {
    BETA * (bound as f32).ln().ln().ln()
}

pub fn ordinary_leaves(limit: usize, prime_array: &Vec<usize>, meissel_cache: &mut HashMap<(usize, usize), usize>) -> isize {
    //debug_assert!(true);
    let mut mu = vec![1isize; limit+1];
    mu[0] = 0;
    for j in 2..=limit {
        if mu[j] == 1 {
            for i in (j..=limit).step_by(j) {
               mu[i] = if mu[i] == 1 {-(j as isize)} else {-mu[i]};
            }
        }
    }
    for j in 2..=int_square_root(limit) {
        if -mu[j] == j as isize {
            let j_squared = j * j;
            for i in (j_squared..=limit).step_by(j_squared) {
                mu[i] = 0;
            }
        }
    }

    let p_c = SMALL_PRIMES[MEISSEL_LOOKUP_SIZE - 1];
    let mut result = 0;
    for i in (2..=limit).filter(|x| mu[*x].abs() > p_c as isize) {
        result += mu[i].signum() * (small_meissel_fn(i, MEISSEL_LOOKUP_SIZE, prime_array, meissel_cache) as isize)
    }

    return result;
}

// pub fn meissel_fn(limit: usize, prime_array: &Vec<usize>, meissel_cache: &mut HashMap<(usize, usize), usize>) -> isize {

// }

pub fn prime_count(x: usize) -> usize {
    x
    // let alpha = get_alpha(x);
    // let lower_bound = (alpha * int_cubic_root(x)) as usize;
    
}

#[cfg(test)]
mod tests {        
    #[test]
    fn test_oleaves() {
        use crate::meissel;
        use std::collections::HashMap;
        let primes = meissel::generate_primes(1000);
        let mut meissel_cache = HashMap::new();
        assert_eq!(meissel::ordinary_leaves(30, &primes, &mut meissel_cache), -5); // Unsure if correct
        assert_eq!(meissel::ordinary_leaves(100, &primes, &mut meissel_cache), -170); // Unsure if correct
    }
}