use std::collections::HashMap;

pub fn letter_combination(digits:&String)->Vec<String>{
    if digits.is_empty() {
        return vec![String::from("")];
    }

    //create hashmap
    let mut hash_group:HashMap<char,Vec<char>> = HashMap::new();
    hash_group.insert('2',vec!['a','b','c']);
    hash_group.insert('3',vec!['d','e','f']);
    hash_group.insert('4',vec!['g','h','i']);
    hash_group.insert('5',vec!['j','k','l']);
    hash_group.insert('6',vec!['m','n','o']);
    hash_group.insert('7',vec!['p','q','r','s']);
    hash_group.insert('8',vec!['t','u','v']);
    hash_group.insert('9',vec!['w','x','y','z']);

    let mut res:Vec<String>=vec![];
    let mut first_time:bool=true;
    for d in digits.chars(){
        if d=='1'{
            continue;
        }
        let mut tmp:Vec<String>=vec![];
        if first_time==true {
            for element in hash_group[&d].iter(){
                tmp.push(element.to_string());
            }
            first_time=false
        } else{ 
            for r in res.iter(){
                for element in hash_group[&d].iter(){
                    let together:String = format!("{}{}", r, element);
                    tmp.push(together);
                }
            }
        }

        res=tmp;
    }
    

    return res
}