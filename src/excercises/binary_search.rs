pub fn binary_search(nums:&Vec<i64>,target:&i64)->i64{
    
    if nums.is_empty() || target<&nums[0] || target>nums.last().unwrap(){
        return -1
    }

    let mut l:usize=0;
    let mut r:usize=nums.len()-1;
    let mut mid:usize;

    while l<=r {
        mid= l + (r-l)/2 ;

        if nums[mid]<*target{
            l=mid+1;
        } else if nums[mid]>*target{
            r=mid-1;
        } else {
            return mid as i64;
        }
    }

    return -1;
}

pub fn search_a_2d_matrix(matrix:&Vec<Vec<i64>>,target:&i64)->bool{
    
    if matrix.is_empty() || *target<matrix[0][0] || target>matrix.last().unwrap().last().unwrap(){
        return false;
    } 

    let mut bot:usize=0;
    let mut top:usize=matrix.len()-1;
    let mut row:usize=0;
    while bot<=top { 
        row=bot + ((top-bot)/2);
        if *target<matrix[row][0] {
            top=row-1;
        } else if *target>matrix[row][0] {
            bot=row+1;
        } else {
            break;
        }
    }

    let mut l:usize=0;
    let mut r:usize=matrix[row].len()-1;
    let mut mid:usize;
    while l<=r {

        mid=l + (r-l)/2;

        if *target<matrix[row][mid]{
            r=mid-1;
        } else if *target>matrix[row][mid]{
            l=mid+1;
        } else {
            return true;
        }
    }

    return false;
}

pub fn koko_eating_bananas(piles:&Vec<i64>,h:&i64)->i64{

    let mut l:i64=1;
    let mut r:i64=*piles.iter().max().unwrap();
    let mut k:i64=0;
    let mut bananas_per_hour:i64;

    while l<=r {
        bananas_per_hour=(l+r)/2;
        
        //time eating bananas using  bananas_per_hour
        let mut total_time:i64=0;
        for p in piles{
            total_time+= ((p-1)/bananas_per_hour)+1
        }

        //change the eating rate
        if total_time<=*h {
            k=bananas_per_hour;
            r=bananas_per_hour-1;
        } else {
            l=bananas_per_hour+1;
        }
    }
    
    return k;
}

pub fn search_in_rotated_sorted_array(nums:&Vec<i64>,target:&i64)->i64{
    
    let mut l:usize=0;
    let mut r:usize=nums.len()-1;
    let mut mid:usize;

    while l<=r {
        mid= l + (r-l)/2 ;
        
        if *target == nums[mid]{
            return mid as i64;
        }

        if nums[l]<=nums[mid]{
            if *target> nums[mid] || *target<nums[l] {
                l=mid+1;
            } else {
                r=mid-1;
            }
        } else {
            if *target < nums[mid] || *target > nums[r] {
                l=mid+1;
            } else {
                r=mid-1;
            }
        }
    }

    return -1;
}
