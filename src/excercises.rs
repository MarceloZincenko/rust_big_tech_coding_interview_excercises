mod array_hashes;
mod permutations;
mod two_pointers;
mod sliding_window;
mod stack;

//ARRAY AND HASHES
pub fn leetcode_217_contains_duplicates(){
    //empty case
    let mut nums:Vec<i64>=Vec::new();
    let mut result:bool=array_hashes::contains_duplicates(&nums);
    assert_eq!(result, false);    
    
    // false case
    nums=vec![1,23,4,5,6,2];
    result=array_hashes::contains_duplicates_2(&nums);
    assert_eq!(result, false);    
    
    // true case
    nums=vec![1,23,4,5,6,2,1];
    result=array_hashes::contains_duplicates_2(&nums);
    assert_eq!(result, true);
}

pub fn leetcode_242_valid_anagram(){
     let s1:String=String::from("anagram");    
     let s2:String=String::from("anagram");
     let s3:String=String::from("anasram");
     
     // true case
     let mut result:bool=array_hashes::valid_anagram(&s1,&s2);
     assert_eq!(result, true);
     
     // false case    
     result=array_hashes::valid_anagram(&s1,&s3);
     assert_eq!(result, false);    
}

pub fn leetcode_1_two_sum(){

    let nums:Vec<i64>=vec![1,23,4,5,6,7,8,9,10];
    let target:i64=16;
    let result:[i64;2]=array_hashes::two_sum(&nums,&target);
    assert_eq!(result, [5,7]);    
}

pub fn leetcode_49_group_anagrams(){

    let strs:Vec<String>=vec![];
    let result:Option<Vec<Vec<String>>>=array_hashes::group_anagrams(&strs);
    let expected=None;
    assert_eq!(result, expected); 

    let strs:Vec<String>=vec![String::from("eat"),String::from("ate"),String::from("ear")];
    let result:Option<Vec<Vec<String>>>=array_hashes::group_anagrams(&strs);
    let mut expected:Vec<Vec<String>>=vec![vec![String::from("eat"),String::from("ate")],vec![String::from("ear")]];
    assert_eq!(result.unwrap().sort(), expected.sort()); 
}

pub fn leetcode_347_top_k_frequent_elements(){

    let nums:Vec<i64>=vec![1,1,1,2,2,3];
    let k:i64=2;
    let result:Vec<i64>=array_hashes::top_k_frequent_elements(&nums,k);
    let expected:Vec<i64>=vec![1,2];
    assert_eq!(result, expected);
}

pub fn leetcode_3238_product_of_array_except_self(){

    //6- Product of array except itself
    let nums:Vec<i64>=vec![1,2,3,4];
    let result:Vec<i64>=array_hashes::product_of_array_except_self(&nums);
    let expected:Vec<i64>=vec![24,12,8,6];
    assert_eq!(result, expected);
}

pub fn leetcode_36_valid_dudoku(){

    let sudoku:[[char;9];9]=[
    ['5','3','.','.','7','.','.','.','.'],
    ['6','.','.','1','9','5','.','.','.'],
    ['.','9','8','.','.','.','.','6','.'],
    ['8','.','.','.','6','.','.','.','3'],
    ['4','.','.','8','.','3','.','.','1'],
    ['7','.','.','.','2','.','.','.','6'],
    ['.','6','.','.','.','.','2','8','.'],
    ['.','.','.','4','1','9','.','.','5'],
    ['.','.','.','.','8','.','.','7','9']];
    let result:bool=array_hashes::valid_sudoku(sudoku);
    assert_eq!(result,true);
}

pub fn leetcode_128_longest_consecutive_sequence(){

    let nums:Vec<i64>=vec![0,3,7,2,5,8,4,6,0,1];
    let result:i64=array_hashes::longest_consecutive_sequence(&nums);
    let expected:i64=9;
    assert_eq!(result, expected);
}

//PERMUTATIONS
pub fn leetcode_17_letter_combination(){
    let digits:String=String::from("23");
    let result:Vec<String>=permutations::letter_combination(&digits);
    let expected:Vec<String>=vec![String::from("ad"),String::from("ae"),String::from("af"),String::from("bd"),String::from("be"),String::from("bf"),String::from("cd"),String::from("ce"),String::from("cf")];
    assert_eq!(result, expected);
}

//TWO POINTERS
pub fn leetcode_125_valid_palindrome(){
    let mut s:String=String::from("A man, a plan, a canal: Panama");    
    
    // true case
    let mut result:bool=two_pointers::valid_palindrome(&s);
    assert_eq!(result, true);
    
    // false case
    s=String::from("no palindrome    on");   
    result=two_pointers::valid_palindrome(&s);
    assert_eq!(result, false);    
}

pub fn leetcode_167_two_sum_ii(){
    let mut nums:Vec<i64>=vec![2,7,11,15];    
    let mut target:i64=9;
    let mut result:[i64;2]=two_pointers::two_sum_ii(&nums,target);
    let mut expected:[i64;2]=[1,2];
    assert_eq!(result, expected);

    nums=vec![2,3,4];    
    target=6;
    result=two_pointers::two_sum_ii(&nums,target);
    expected=[1,3];
    assert_eq!(result, expected);   
}

pub fn leetcode_15_three_sum(){
    let mut nums:Vec<i64>=vec![-1,0,1,2,-1,-4];    
    let result:Vec<Vec<i64>>=two_pointers::three_sum(&mut nums);
    let expected:Vec<Vec<i64>>=vec![vec![-1,-1,2],vec![-1,0,1]];
    assert_eq!(result, expected);
}

pub fn leetcode_11_container_with_most_water(){
    let nums:Vec<i64>=vec![1,8,6,2,5,4,8,3,7];    
    let result:i64=two_pointers::container_with_most_water(&nums);
    let expected:i64=49;
    assert_eq!(result, expected);
}

pub fn leetcode_5_longest_palindromic_substring(){
    let s:String=String::from("babad");    
    let result:String=two_pointers::longest_palindromic_substring(&s);
    let expected:String=String::from("bab");    
    assert_eq!(result, expected);
}

//SLIDING WINDOW
pub fn leetcode_14_longest_common_prefix(){
    let strs:Vec<String>=vec![String::from("flower"),String::from("flow"),String::from("fligth")];
    let result:String=sliding_window::longest_common_prefix(&strs);
    let expected:String=String::from("fl");
    assert_eq!(result, expected);
}

pub fn leetcode_121_best_time_to_buy_and_sell_stock(){
    let prices:Vec<i64>=vec![7,1,5,3,6,4];
    let result:i64=sliding_window::best_time_to_buy_and_sell_stock(&prices);
    let expected:i64=5;
    assert_eq!(result,expected);
}

pub fn leetcode_3_longest_substring_without_repeating_characters(){
    let s:String= "abcabcbb".to_string();
    let result:i64=sliding_window::longest_substring_without_repeating_characters(&s);
    let expected:i64=3;
    assert_eq!(result,expected);
}

pub fn leetcode_424_longest_repeating_character_replacement(){
    let s:String= "AABABBA".to_string();
    let k:i64=1;
    let result:i64=sliding_window::longest_repeating_character_replacemen(&s,k);
    let expected:i64=4;
    assert_eq!(result,expected);
}

pub fn leetcode_567_permutation_in_string(){
    let s1:String="ab".to_string();
    let s2:String="eidbaooo".to_string();
    let result:bool=sliding_window::permutation_in_string(&s1, &s2);
    let expected:bool=true;
    assert_eq!(result,expected);
}

pub fn leetcode_239_sliding_window_maximum(){
    let nums:Vec<i64>=vec![1,3,-1,-3,5,3,6,7];
    let k:i64=3;
    let result:Vec<i64>=sliding_window::sliding_window_maximum(&nums,&k);
    let expected:Vec<i64>=vec![3,3,5,5,6,7];
    assert_eq!(result,expected);
}

pub fn local_minimum(){
    let nums:Vec<i64>=vec![1,3,-1,-3,5,3,6,7];
    let k:i64=3;
    let result:Vec<i64>=sliding_window::local_minimum(&nums,&k);
    let expected:Vec<i64>=vec![-1,-3,-3,-3,3,3];
    assert_eq!(result,expected);
}


//STACK
pub fn leetcode_20_valid_parentheses(){
    let chars:Vec<char>=vec!['(','(',')','[',']','{','}',')','[','(',')','[',']','{','}',']','{','(',')','[',']','{','}','}'];
    let result:bool=stack::valid_parentheses(&chars);
    let expected:bool=true;
    assert_eq!(result,expected);
}

pub fn leetcode_739_daily_temperatures(){
    let temperatures:Vec<i64>=vec![73,74,75,71,69,72,76,73];
    let result:Vec<i64>=stack::daily_temperatures(&temperatures);
    let expected:Vec<i64>=vec![1,1,4,2,1,1,0,0];
    assert_eq!(result, expected);
}

pub fn leetcode_150_evaluate_reverse_polish_notation(){
    let temperatures:Vec<String>=vec!["10".to_string(),"6".to_string(),"9".to_string(),"3".to_string(),"+".to_string(),"-11".to_string(),"*".to_string(),"/".to_string(),"*".to_string(),"17".to_string(),"+".to_string(),"5".to_string(),"+".to_string()];
    let result:i64=stack::evaluate_reverse_polish_notation(&temperatures);
    let expected:i64=22;
    assert_eq!(result, expected);
}