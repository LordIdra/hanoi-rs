pub type Arrangement = Vec<Vec<i32>>;

pub fn get_initial_arrangement() -> Arrangement {
    vec![vec![1, 2, 3], vec![], vec![]]
}

pub fn is_final_arrangement(arrangement: &Arrangement) -> bool {
    // Skip first platform
    for platform in arrangement.iter().skip(1) {
        if platform.len() != 3 {
            continue;
        }
        if platform[0] == 1 && platform[1] == 2 && platform[2] == 3 {
            return true;
        }
    }
    false
}