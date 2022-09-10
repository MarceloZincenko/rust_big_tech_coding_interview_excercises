use std::collections::HashMap;

pub fn valid_parentheses(chars:&Vec<char>)->bool{
    //Closing Hash Map
    let mut hash_map:HashMap<char,char>=HashMap::new();
    hash_map.entry(')').or_insert('(');
    hash_map.entry('}').or_insert('{');
    hash_map.entry(']').or_insert('[');
    
    //the stack
    let mut stack:Vec<char>=vec![];
    
    for c in chars.iter(){
        if hash_map.contains_key(c){
            if stack.len()>0 && stack.last()==hash_map.get(c){
                stack.pop();
            } else {
                return false;
            }
        } else {
            stack.push(*c);
        }
    }
    return stack.len()==0
}

pub fn evaluate_reverse_polish_notation(strings:&Vec<String>)->i64{
    let mut stack:Vec<i64>=vec![];

    for s in strings.iter(){
        if s=="+"{
            let second:i64=stack.pop().unwrap();
            let first:i64=stack.pop().unwrap();
            stack.push(first+second);
        } else if s=="*"{
            let second:i64=stack.pop().unwrap();
            let first:i64=stack.pop().unwrap();
            stack.push(first*second);
        } else if s=="-"{
            let second:i64=stack.pop().unwrap();
            let first:i64=stack.pop().unwrap();
            stack.push(first-second);
        } else if s=="/"{
            let second:i64=stack.pop().unwrap();
            let first:i64=stack.pop().unwrap();
            stack.push(first/second);
        } else {
            stack.push(s.parse::<i64>().unwrap());
        }
    }

    return stack[0];
}

pub fn daily_temperatures(temperatures:&Vec<i64>)->Vec<i64>{
    let mut stack:Vec<(i64,i64)>=vec![];
    let mut res:Vec<i64>=vec![0; temperatures.len()];

    for (i,t) in temperatures.iter().enumerate() {
        
        while stack.is_empty()==false && *t>stack.last().unwrap().1 {
            let element:Option<(i64,i64)>=stack.pop();
            let position:usize=element.unwrap().0 as usize;
            res[position]=(i as usize - position) as i64;
        }
        stack.push((i as i64,*t as i64));       
    }

    return res;
}
