mod search;

pub fn binary_search(){   
    let nums:Vec<i64>=vec![1,2,3,4,5,6,7,8,9,10];
    let result:i64=search::binary_search(&nums,7);
    assert_eq!(result, 6);    
}