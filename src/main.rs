mod rope_bridge;

fn main() {
    let total = rope_bridge::count_distinct_tail_positions().unwrap();
    println!("Total: {}", total);
}

// fn execute_second_first_part() {
//     let score = rock_paper_scissors::second_part::compute_score_with_second_strategy();
//     print!("Score obtained: {} ", score);
// }

// fn execute_first() {
//     let max_callory = callories::find_elf_with_most_callories();
//     let max_callory_v2 = callories::find_elf_with_most_callories_v2();
//     let sum = callories::find_sum_of_three_most_callories();
//     let sum_v2 = callories::find_sum_of_three_most_callories_v2();
//     println!("The max callories: {}", max_callory);
//     println!("The max callories v2: {}", max_callory_v2);
//     println!("The sum of three most callories: {}", sum);
//     println!("The sum of three most callories_v2: {}", sum_v2);
// }
