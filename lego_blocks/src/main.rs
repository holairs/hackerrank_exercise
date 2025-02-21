fn legoBlocks(n: i32, m: i32) -> i32 {
    const MOD: u64 = 1_000_000_007;

    if n <= 0 || m <= 0 {
        return 0;
    }

    let (n_u64, m_usize) = (n as u64, m as usize);

    let mut row_ways = vec![0u64; m_usize + 1];
    row_ways[0] = 1;
    for w in 1..=m_usize {
        let mut total = 0u64;
        for block_len in 1..=4 {
            if w >= block_len {
                total = (total + row_ways[w - block_len]) % MOD;
            }
        }
        row_ways[w] = total;
    }

    fn mod_exp(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
        let mut result = 1u64;
        base %= modulus;
        while exp > 0 {
            if exp & 1 == 1 {
                result = (result * base) % modulus;
            }
            base = (base * base) % modulus;
            exp >>= 1;
        }
        result
    }

    let mut T = vec![0u64; m_usize + 1];
    for w in 1..=m_usize {
        T[w] = mod_exp(row_ways[w], n_u64, MOD);
    }

    let mut S = vec![0u64; m_usize + 1];
    S[1] = T[1];
    for w in 2..=m_usize {
        let mut sum_breaks = 0u64;
        for k in 1..w {
            sum_breaks = (sum_breaks + (S[k] * T[w - k]) % MOD) % MOD;
        }
        S[w] = if T[w] >= sum_breaks {
            (T[w] - sum_breaks) % MOD
        } else {
            (T[w] + MOD - sum_breaks) % MOD
        };
    }

    (S[m_usize] % MOD) as i32
}
