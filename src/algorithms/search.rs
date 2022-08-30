
pub fn binary_search(nums:&Vec<i64>,x:i64)->i64{
    let mut l:usize=0;
    let mut r:usize=(nums.len()-1) as usize;
    let mut mid:usize;

    while l<=r {
        mid=l + ((r-l)/2);
        if nums[mid] == x {
            return mid as i64;
        } else if nums[mid] < x {
            l=mid+1;
        } else {
            r=mid-1;
        } 
    }

    return -1 as i64;

}