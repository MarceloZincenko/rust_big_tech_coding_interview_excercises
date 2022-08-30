use std::cmp;

pub fn valid_palindrome(s:&String)->bool{

    //two pointer: l,r
    let mut l:usize=0;
    let mut r:usize=s.len()-1;
    let mut left:char = s.as_str().chars().nth(l).unwrap();
    let mut rigth:char = s.as_str().chars().nth(r).unwrap();

    while l<r {

        while l<r && left.is_alphanumeric()==false {
            l+=1;
            left = s.as_str().chars().nth(l).unwrap();
        }
        while l<r &&  rigth.is_alphanumeric()==false {
            r-=1;
            rigth = s.as_str().chars().nth(r).unwrap();
        }
        if left.to_lowercase().to_string()!=rigth.to_lowercase().to_string() {
            return false;
        }
        l+=1;
        left = s.as_str().chars().nth(l).unwrap();
        r-=1;
        rigth = s.as_str().chars().nth(r).unwrap();
    }
    return true;
}

pub fn two_sum_ii(nums:&Vec<i64>,target:i64)->[i64;2]{

    //two pointer: l,r
    let mut l:usize=0;
    let mut r:usize=nums.len()-1;
    while l<r {
        if nums[l]+nums[r]>target{
            r-=1;
        } else if nums[l]+nums[r]<target{
            l+=1;
        } else {
            return [l as i64 +1 ,r as i64 +1];
        }
    }
    
    return [-1 ,-1];
}

pub fn three_sum(nums:&mut Vec<i64>)->Vec<Vec<i64>>{

    let mut res:Vec<Vec<i64>>=vec![];
    nums.sort();
    let mut i:usize=0;

    for n in nums.iter(){
        if i==0 || nums[i-1]==nums[i]{
            i+=1;
            continue
        }
        let mut l:usize=i+1;
        let mut r:usize=nums.len()-1;
        while l<r{
            let suma:i64=*n+nums[l]+nums[r];
            if suma>0 {
                r-=1;
            } else if suma <0 {
                l+=1;
            }
            else{
                res.push(vec![*n,nums[l],nums[r]]);
                l+=1;
                while nums[l]==nums[l-1] && l>r{
                    l+=1;
                }
            }
        }
        i+=1;
    }

    if res.is_empty(){
        return vec![vec![]];
    } else {
        return res;
    }
}

pub fn container_with_most_water(height:&Vec<i64>)->i64{
    let mut res:i64=0;
    let mut l:usize=0;
    let mut r:usize=height.len()-1;

    while l<r {
        res=cmp::max(res,(r as i64 - l as i64)*cmp::min(height[l],height[r]));
        if height[l]<height[r] {
            l+=1;
        } else {
            r-=1;
        }
    }
    return res;
}

pub fn longest_palindromic_substring(s:&String)->String{
    
    let mut res:String=String::from("");
    let mut length:isize=0;
    let mut l:isize;
    let mut r:isize;

    //odd case
    for i in 0..s.len(){
        l=i as isize;
        r=i as isize;
        while l>=0 && r<s.len() as isize && s.as_str().chars().nth(l as usize).unwrap()==s.as_str().chars().nth(r as usize).unwrap(){
            l-=1;
            r+=1;
        }

        if r-l+1>length{
            let start:usize=(l+1) as usize;
            let end:usize=r as usize;
            res=s[start..end].to_string();
            length=r-l+1;
        }
    }
    
    //even case
    for i in 0..s.len(){
        l=i as isize;
        r=i as isize +1;
        while l>=0 && r<s.len() as isize && s.as_str().chars().nth(l as  usize).unwrap()==s.as_str().chars().nth(r as usize).unwrap(){
            l-=1;
            r+=1;
        }

        if r-l+1>length{ 
            let start:usize=(l+1) as usize;
            let end:usize=r as usize;
            res=s[start..end].to_string();
            length=r-l+1;
        }
    }
    return res

}