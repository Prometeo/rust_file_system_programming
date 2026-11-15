use std::cmp::Ordering;
use std::path::PathBuf;
use std::time::SystemTime;
// We define a struct to hold all the data needed for ranking.
#[derive(Debug)]
struct SearchResult {
    path: PathBuf,
    match_count: usize,
    last_modified: SystemTime,
}

fn rank_and_sort_results(mut results: Vec<SearchResult>) {
    // We use the sort_by method provided by the standard library Vector.
    // This method takes a closure that compares two items (a and b).
    results.sort_by(|a, b| {
        // Step 1: Compare by match count (Relevance).
        // We want the highest match count first, so we compare 'b' to 'a'
        // to create a descending order.
        let count_comparison = b.match_count.cmp(&a.match_count);
        // We check the result of the count comparison.
        match count_comparison {
            // If the counts are different, this is our final decision.
            // If b has a higher count, b comes first. If a has a higher count, a comes first.
            Ordering::Less | Ordering::Greater => count_comparison,
            // Step 2: Tie-breaker.
            // If both files have the exact same match count, we fall back to Recency.
            Ordering::Equal => {
                // We compare the SystemTime timestamps.
                // Again, we compare 'b' to 'a' because we want the newest (largest) time first.
                b.last_modified.cmp(&a.last_modified)
            }
        }
    });
    // After sort_by completes, the Vector is mutated in place and strictly ordered.
    println!("--- Ranked Search Results ---");
    for (index, result) in results.iter().enumerate() {
        // We format the output nicely for the user.
        println!(
            "{}. Matches: {:<4} | Path: {:?}",
            index + 1,
            result.match_count,
            result.path
        );
    }
}

// A simple simulation to demonstrate the sorting logic.
fn execute_ranking_simulation() {
    let mut simulated_results = Vec::new();
    let current_time = SystemTime::now();
    // We add a highly relevant file.
    simulated_results.push(SearchResult {
        path: PathBuf::from("main_config.txt"),
        match_count: 50,
        last_modified: current_time, // Modified just now
    });
    // We add an old file with a low match count.
    simulated_results.push(SearchResult {
        path: PathBuf::from("old_backup.txt"),
        match_count: 2,
        last_modified: current_time - std::time::Duration::from_secs(86400 * 30), // 30 days ago
    });
    // We add a new file that has the exact same low match count, creating a tie.
    simulated_results.push(SearchResult {
        path: PathBuf::from("recent_notes.txt"),
        match_count: 2,
        last_modified: current_time, // Modified just now
    });
    // We pass our simulated list to the ranking function.
    // The expected output order should be:
    // 1. main_config.txt (Highest relevance)
    // 2. recent_notes.txt (Tied relevance, but newer recency)
    // 3. old_backup.txt (Tied relevance, but older recency)
    rank_and_sort_results(simulated_results);
}

fn main() {
    println!("Hello, world!");
}
