
    // Like the Meissel function, but computes the number of numbers
    //  less than m that are coprime to the first n prime numbers,
    //  and have *exactly* two distinct prime factors (n.b. squarefree)
    // Using algorithm similar to the one on page 760 and 765 here:
    //  http://sweet.ua.pt/tos/bib/5.4.pdf
    fn meissel2_fn(&self, m: usize, n: usize) -> usize {
        let mut result = (n * (n - 1)) / 2;
        let mut u = util::int_square_root(m);
        let mut v = n;
        let mut w = u + 1;
        while u > n {
            let mut interval = vec![true; m / n];
            if u < w {
                w = cmp::max(2, u - n);
            }
            if self.primes.contains(&u) {
                let y = m / u;
            }
        }
        result -= (v * (v - 1)) / 2;
        return result;
    }