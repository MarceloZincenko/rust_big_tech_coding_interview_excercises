use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
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

pub fn permutation_in_string(s1:&String,s2:&String)->bool{
    if s1.len()>s2.len(){
        return false;
    }
    
    //Build s1 hash map
    let mut hash_s1:HashMap<char,i8>=HashMap::new();
    for c in s1.chars(){
        let freq = hash_s1.entry(c).or_insert(0);
        *freq += 1;
    }

    //Build s2 hash map
    let window_size:usize=s1.len();
    let mut hash_s2:HashMap<char,i8>=HashMap::new();
    for i in 0..window_size{
        let freq = hash_s2.entry(s2.as_str().chars().nth(i).unwrap().clone()).or_insert(0);
        *freq += 1;
    }

    //Compare hash_s1 with hash_s2
    if hash_s1.eq(&hash_s2){
        return true;
    }

    //We continue searching
    for i in window_size..s2.len(){
        // add new character
        let freq = hash_s2.entry(s2.as_str().chars().nth(i).unwrap()).or_insert(0);
        *freq += 1;
        // substract previous character 
        let freq = hash_s2.entry(s2.as_str().chars().nth(i-window_size).unwrap()).or_insert(0);
        *freq -= 1;
        if hash_s2.get(&s2.as_str().chars().nth(i-window_size).unwrap())==Some(&0){
            hash_s2.remove(&s2.as_str().chars().nth(i-window_size).unwrap());
        }

        if hash_s1.eq(&hash_s2){
            return true;
        }
        
    }

    return false

}

pub fn sliding_window_maximum(nums:&Vec<i64>,k:&i64)->Vec<i64>{
        let mut indexes:VecDeque<usize>=VecDeque::new();
        let mut values:VecDeque<i64>=VecDeque::new();
        let mut results:Vec<i64>=vec![];

        for (i,v) in nums.iter().enumerate(){
            while values.is_empty()==false && v>values.get(values.len()-1).unwrap(){
                indexes.pop_back();
                values.pop_back();
            }
            indexes.push_back(i);
            values.push_back(*v);
            while indexes.is_empty()==false && ((*indexes.get(0).unwrap()) as i64) < ( i as i64 + 1 - *k){
                indexes.pop_front();
                values.pop_front();
            }
            
            if i+1>=*k as usize{
                results.push(*values.get(0).unwrap())
            }
        }
    
    
    return results;
}

pub fn local_minimum(nums:&Vec<i64>,k:&i64)->Vec<i64>{
    let mut indexes:VecDeque<usize>=VecDeque::new();
    let mut values:VecDeque<i64>=VecDeque::new();
    let mut results:Vec<i64>=vec![];

    for (i,v) in nums.iter().enumerate(){
        while values.is_empty()==false && v<values.get(values.len()-1).unwrap(){
            indexes.pop_back();
            values.pop_back();
        }
        indexes.push_back(i);
        values.push_back(*v);
        while indexes.is_empty()==false && ((*indexes.get(0).unwrap()) as i64) < ( i as i64 + 1 - *k){
            indexes.pop_front();
            values.pop_front();
        }
        
        if i+1>=*k as usize{
            results.push(*values.get(0).unwrap())
        }
    }


return results;
}