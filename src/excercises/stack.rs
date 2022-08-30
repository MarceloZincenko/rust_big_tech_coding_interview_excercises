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