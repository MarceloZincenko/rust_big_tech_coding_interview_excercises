use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp;

pub fn longest_common_prefix(strs:&Vec<String>)->String{
    let mut res:String=String::from("");

    for (i,c) in strs[0].chars().enumerate(){
        for string in strs {
            if i>=string.len() || string.as_str().chars().nth(i).unwrap()!=c{
                return res;
            }
        }
        res=format!("{}{}", res, c.to_string());
    } 

    return res;
}

pub fn best_time_to_buy_and_sell_stock(prices:&Vec<i64>)->i64{
    let mut res:i64=0;
    
    let mut l:usize=0;
    for r in 0..prices.len(){
        if prices[r]<prices[l]{
            l=r;
        }
        res=cmp::max(res,prices[r]-prices[l])
    }   

    return res;
}

pub fn longest_substring_without_repeating_characters(s:&String)->i64{
    let mut char_set:HashSet<char>=HashSet::new();
    let mut res:i64=0;
    
    let mut l:usize=0;
    for r in 0..s.len(){
        while char_set.contains(&s.as_str().chars().nth(r).unwrap()) {
            char_set.remove(&s.as_str().chars().nth(l).unwrap().clone());
            l+=1;
        }
        char_set.insert(s.as_str().chars().nth(r).unwrap().clone());
        res=cmp::max(res,(r-l+1) as i64);
    }   

    return res;
}

pub fn longest_repeating_character_replacemen(s:&String,k:i64)->i64{
    let mut char_map:HashMap<char,i64>=HashMap::new(); //key: char and value:count
    let mut res:i64=0;

    let mut l:usize=0;
    let mut max_equal_letters:i64=0;
    for  r in 0..s.len(){
        let freq = char_map.entry(s.as_str().chars().nth(r).unwrap().clone()).or_insert(0);
        *freq += 1;
        max_equal_letters=cmp::max(max_equal_letters,freq.clone());
        if (r-l+1) as i64 - max_equal_letters > k {
            let freq = char_map.entry(s.as_str().chars().nth(l).unwrap().clone()).or_insert(0);
            *freq -= 1;
            l+=1;
        }
        res=cmp::max(res,(r-l+1) as i64); 
    }

    return res;
}