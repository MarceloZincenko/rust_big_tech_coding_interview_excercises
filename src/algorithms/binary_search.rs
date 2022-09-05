
pub fn binary_search(nums:&Vec<i64>,x:i64)->i64{
    let mut l:isize=0;
    let mut r:isize=(nums.len()-1) as isize;
    let mut mid:isize;

    while l<=r {
        mid=(r+l)/2;
        if nums[mid as usize] == x {
            return mid as i64;
        } else if nums[mid as usize] < x {
            l=mid+1;
        } else {
            r=mid-1;
        } 
    }

    return -1 as i64;
}

pub fn binary_search_first_greater_than(nums:&Vec<i64>,x:i64)->i64{
    let mut l:isize=0;
    let mut r:isize=(nums.len()-1) as isize;
    let mut mid:isize;
    let mut ans:i64;

    ans=-1;
    while l<=r {
        
        mid=(r+l)/2;
        
        if nums[mid as usize] <= x {
            l=mid+1;
        } else {
            ans=mid as i64;
            r=mid-1;
        } 
    }

    return ans;
}

pub fn binary_search_greater_or_equal_than(nums:&Vec<i64>,x:i64)->i64{
    
    let mut ans:i64=binary_search(&nums,x);
    if ans==-1{
        ans=binary_search_first_greater_than(&nums,x);
    }
    
    return ans;
}

pub fn binary_search_first_lower_than(nums:&Vec<i64>,x:i64)->i64{
    let mut l:isize=0;
    let mut r:isize=(nums.len()-1) as isize;
    let mut mid:isize;
    let mut ans:i64;

    ans=-1;
    while l<=r {
        
        mid=(r+l)/2;
        
        if nums[mid as usize] >= x {
            r=mid-1;
        } else {
            ans=mid as i64;
            l=mid+1;
        } 
    }

    return ans;
}

pub fn binary_search_lower_or_equal_than(nums:&Vec<i64>,x:i64)->i64{
    let mut ans:i64=binary_search(&nums,x);
    if ans==-1{
        ans=binary_search_first_lower_than(&nums,x);
    }
    
    return ans;
}