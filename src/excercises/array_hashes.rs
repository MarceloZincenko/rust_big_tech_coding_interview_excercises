use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::cmp;

pub fn contains_duplicates(nums:&Vec<i64>)->bool{
    
    let mut set_nums:HashSet<&i64>= HashSet::new();
    for n in nums.iter() {
        if set_nums.contains(&n){
            return true;
        }
        set_nums.insert(&n);
    }
    return false;
}

pub fn contains_duplicates_2(nums:&Vec<i64>)->bool{
    let hash_set:HashSet<i64> =nums.iter().cloned().collect();
     return nums.len()!=hash_set.len();
}

fn build_char_hash(s:&String)->HashMap<char,i64>{
    let mut result:HashMap<char,i64> = HashMap::new();
    
    for c in s.chars(){
        let count = result.entry(c).or_insert(0); // or_insert method returns a mutable reference (&mut V) to the value for the specified key. 
        *count += 1; // dereference to add 1
    } //he mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.

    //println!("{:?}", result);
    return  result;
}


pub fn valid_anagram(s:&String,t:&String)->bool{
     if s.len()!=t.len() {
        return false;
     }

    let hash_s=build_char_hash(&s);
    let hash_t=build_char_hash(&t);
    return hash_s==hash_t;

}

pub fn two_sum(nums:&Vec<i64>,target:&i64)->[i64;2]{
    let mut hash_diff_position:HashMap<i64,i64> = HashMap::new(); // diff->position
    
    for (i,v) in nums.iter().enumerate() {
        let diff:i64=target-v;
        if hash_diff_position.contains_key(&diff)==true {
            return [*hash_diff_position.get(&diff).unwrap(),i as i64];
        }
        hash_diff_position.entry(*v).or_insert(i as i64);
    }

    return [-1,-1];

}

pub fn group_anagrams(strs:&Vec<String>)->Option<Vec<Vec<String>>>{

    if strs.is_empty() {
        return None;
    }

    let mut hash_group:HashMap<[i32;26],Vec<String>> = HashMap::new(); // diff->position

    for s in strs {
        
        let mut str_array:[i32;26]=[0;26];
        
        for c in s.chars() {
            let str_init:char='a';
            let init:u32=str_init.into();
            let end:u32=c.into();
            str_array[end as usize - init as usize]+=1;
        }

        if hash_group.contains_key(&str_array){
            hash_group.get_mut(&str_array).unwrap().push(s.clone());
        } else{
            hash_group.insert(str_array,vec![s.clone()]);
        }

    }
    
    let mut result:Vec<Vec<String>>=vec![];
    
    for (_key,value) in hash_group{
        result.push(value);
    }
    
    return Some(result);

}

pub fn top_k_frequent_elements(nums:&Vec<i64>,k:i64)->Vec<i64>{
    
    //hashFreq
    let mut hash_freq:HashMap<i64,i64>=HashMap::new();
    for n in nums {
        let freq = hash_freq.entry(*n).or_insert(0);
        *freq += 1;
    }

    //now I build array of empty arrays of length nums
    let mut frequency_array:Vec<Vec<i64>>=vec![vec![];nums.len()+1];
    for (key,value) in hash_freq{
        frequency_array[value as usize].push(key);
    }
    //println!("{:?}",frequency_array);

    //get the result
    let mut res:Vec<i64>=vec![];
    for element in frequency_array.iter().rev(){
        if !element.is_empty(){
            for n in element.iter(){
                res.push(*n);
                if res.len() as i64==k {
                    return res;
                }
            }
        }
    }

    return res;
}

pub fn product_of_array_except_self(nums:&Vec<i64>)->Vec<i64>{
    let mut res:Vec<i64>=vec![1;nums.len()];

    let mut tmp:i64=1;
    for (i,n) in nums.iter().enumerate(){
        res[i]=tmp;
        tmp*=n;
    }
    tmp=1;
    for (i,n) in nums.iter().enumerate().rev(){
        res[i]*=tmp;
        tmp*=n;
    }

    return res;
}

pub fn valid_sudoku(sudoku:[[char;9];9])->bool{
    //create hashes
    let mut set_row:HashSet<char>;
    let mut hash_column:HashMap<usize,HashSet<char>>=HashMap::new();
    let mut hash_squeare:HashMap<(i64,i64),HashSet<char>>=HashMap::new();
    
    //start iteration
    for (row_num,row_array) in sudoku.iter().enumerate(){
        set_row=HashSet::new();
        for (col_num,value) in row_array.iter().enumerate(){
            if *value=='.'{
                continue
            }
            let squeare_row:i64=row_num as i64 /3 as i64;
            let squeare_col:i64=col_num as i64 /3 as i64;
            if set_row.contains(value) || (hash_column.contains_key(&col_num) && hash_column.get(&col_num).unwrap().contains(value)) || (hash_squeare.contains_key(&(squeare_row,squeare_col)) && hash_squeare.get(&(squeare_row,squeare_col)).unwrap().contains(value)){
                return false
            }

            set_row.insert(*value);
            //insert in set inside hashmap
            let e = hash_column.entry(col_num);
            e.or_default().insert(*value);
            //insert in set inside hashmap
            let e = hash_squeare.entry((squeare_row,squeare_col));
            e.or_default().insert(*value);
        }
    }
    return true;
}

pub fn longest_consecutive_sequence(nums:&Vec<i64>)->i64{
    let set_nums:HashSet<i64>=HashSet::from_iter(nums.iter().cloned());
    let mut max_length:i64=1;

    for n in nums.iter(){
        let mut length:i64=0;
        let previous:i64=n-1;
        if set_nums.contains(&previous){
            continue
        }
        let mut search:i64=*n;
        while  set_nums.contains(&search){
            length+=1;
            search+=1;
        }
        max_length=cmp::max(max_length,length);
    }

    return max_length;
}