pub fn edit_distance(source: &str, target: &str) -> usize {
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    let m = source_chars.len();
    let n = target_chars.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..= m {
        dp[i][0] = i;
    }
    for j in 0..= n {
        dp[0][j] = j;
    }

    for i in 1..= m {
        for j in 1..= n {
            let cost_subst = if source_chars[i - 1] == target_chars[j - 1] { 0 } else { 1 };
            let substitution = dp[i - 1][j - 1] + cost_subst;
            let insertion = dp[i][j - 1] + 1;
            let deletion = dp[i - 1][j] + 1;
            dp[i][j] = substitution.min(insertion).min(deletion);
        }
    }
    dp[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;

     #[test]
    fn test_distance() {
        assert_eq!(edit_distance("gumbo", "gambol"), 2);
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        assert_eq!(edit_distance("rosettacode", "raisethysword"), 8);
    }
}
