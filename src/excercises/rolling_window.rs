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
