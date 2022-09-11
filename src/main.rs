mod excercises;
mod algorithms;

fn main() {

    //EXCERCISES
    //Array and Hashes:
    excercises::leetcode_1_two_sum();
    excercises::leetcode_36_valid_dudoku();
    excercises::leetcode_49_group_anagrams();
    excercises::leetcode_128_longest_consecutive_sequence();
    excercises::leetcode_217_contains_duplicates();
    excercises::leetcode_242_valid_anagram();
    excercises::leetcode_347_top_k_frequent_elements();
    excercises::leetcode_3238_product_of_array_except_self();

    //backtracking
    excercises::leetcode_17_letter_combination();

    //Two pointers
    excercises::leetcode_5_longest_palindromic_substring();
    excercises::leetcode_11_container_with_most_water();
    excercises::leetcode_15_three_sum();
    excercises::leetcode_125_valid_palindrome();
    excercises::leetcode_167_two_sum_ii();

    //Sliding Window
    excercises::leetcode_3_longest_substring_without_repeating_characters();
    excercises::leetcode_14_longest_common_prefix();
    excercises::leetcode_121_best_time_to_buy_and_sell_stock();
    excercises::leetcode_424_longest_repeating_character_replacement();
    excercises::leetcode_567_permutation_in_string();
    excercises::leetcode_239_sliding_window_maximum();
    excercises::local_minimum();
    
    //Stack
    excercises::leetcode_20_valid_parentheses();
    excercises::leetcode_150_evaluate_reverse_polish_notation();
    excercises::leetcode_739_daily_temperatures();

    //Binary Search
    excercises::leetcode_33_search_in_rotated_sorted_array();
    excercises::leetcode_74_search_a_2d_matrix();
    excercises::leetcode_704_binary_search();
    excercises::leetcode_875_koko_eating_bananas();

    //ALGORITHMS
    //binary search
    algorithms::binary_search();
    algorithms::binary_search_first_greater_than();
    algorithms::binary_search_greater_or_equal_than();
    algorithms::binary_search_first_lower_than();
    algorithms::binary_search_lower_or_equal_than();


}
