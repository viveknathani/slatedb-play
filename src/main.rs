fn partition_sums(weights: &[u64], cuts: &[usize]) -> Vec<u64> {
    let mut result = Vec::new();
    let mut start = 0;

    for &cut in cuts {
        result.push(weights[start..cut].iter().sum());
        start = cut;
    }

    result.push(weights[start..].iter().sum());
    result
}

// Greedy approximation: cut once accumulated work crosses target.
// Returns cut positions where each cut is an index into `weights`.
fn greedy_partition(weights: &[u64], k: usize) -> Vec<usize> {
    if k <= 1 || weights.is_empty() {
        return vec![];
    }

    let total: u64 = weights.iter().sum();
    let target = total.div_ceil(k as u64);

    let mut cuts = Vec::new();
    let mut acc = 0;

    // Need at least one remaining item per remaining partition.
    for i in 0..weights.len() {
        acc += weights[i];

        let remaining_items = weights.len() - (i + 1);
        let remaining_parts = k - cuts.len() - 1;

        if cuts.len() + 1 < k && acc >= target && remaining_items >= remaining_parts {
            cuts.push(i + 1);
            acc = 0;
        }
    }

    cuts
}

// Optimal linear partition DP.
// Minimizes the largest partition sum over k contiguous non-empty partitions.
// Returns cut positions where each cut is an index into `weights`.
fn linear_partition_dp(weights: &[u64], k: usize) -> Vec<usize> {
    let n = weights.len();

    if k <= 1 || n <= 1 {
        return vec![];
    }

    let k = k.min(n);

    let mut prefix = vec![0u64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + weights[i];
    }

    let mut dp = vec![vec![u64::MAX; k + 1]; n + 1];
    let mut div = vec![vec![0usize; k + 1]; n + 1];

    dp[0][0] = 0;

    for i in 1..=n {
        dp[i][1] = prefix[i];
    }

    for parts in 2..=k {
        for i in parts..=n {
            for x in (parts - 1)..i {
                let left = dp[x][parts - 1];
                let right = prefix[i] - prefix[x];
                let cost = left.max(right);

                if cost < dp[i][parts] {
                    dp[i][parts] = cost;
                    div[i][parts] = x;
                }
            }
        }
    }

    let mut cuts = Vec::new();
    let mut i = n;
    let mut parts = k;

    while parts > 1 {
        let cut = div[i][parts];
        cuts.push(cut);
        i = cut;
        parts -= 1;
    }

    cuts.reverse();
    cuts
}

fn print_comparison(weights: &[u64], k: usize) {
    println!("weights = {:?}, k = {}", weights, k);

    let greedy_cuts = greedy_partition(weights, k);
    let greedy_sums = partition_sums(weights, &greedy_cuts);
    let greedy_max = greedy_sums.iter().max().unwrap_or(&0);

    println!("greedy cuts: {:?}, sums: {:?}, max: {}", greedy_cuts, greedy_sums, greedy_max);

    let dp_cuts = linear_partition_dp(weights, k);
    let dp_sums = partition_sums(weights, &dp_cuts);
    let dp_max = dp_sums.iter().max().unwrap_or(&0);

    println!("dp cuts:     {:?}, sums: {:?}, max: {}", dp_cuts, dp_sums, dp_max);
    println!();
}


// STDOUT:
// weights = [15, 15, 15, 15, 40], k = 3
// greedy cuts: [3], sums: [45, 55], max: 55
// dp cuts:     [2, 4], sums: [30, 30, 40], max: 40

// weights = [10, 10, 10, 10, 10, 50], k = 3
// greedy cuts: [4], sums: [40, 60], max: 60
// dp cuts:     [2, 5], sums: [20, 30, 50], max: 50

// weights = [1, 1, 1, 97], k = 4
// greedy cuts: [], sums: [100], max: 100
// dp cuts:     [1, 2, 3], sums: [1, 1, 1, 97], max: 97

// weights = [100, 300, 600, 500, 300], k = 3
// greedy cuts: [3], sums: [1000, 800], max: 1000
// dp cuts:     [2, 3], sums: [400, 600, 800], max: 800

fn main() {
    print_comparison(&[15, 15, 15, 15, 40], 3);
    print_comparison(&[10, 10, 10, 10, 10, 50], 3);
    print_comparison(&[1, 1, 1, 97], 4);
    print_comparison(&[100, 300, 600, 500, 300], 3);
}

