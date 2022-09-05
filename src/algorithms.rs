mod binary_search;

pub fn binary_search(){   
    let nums:Vec<i64>=vec![1,2,3,4,5,6,7,8,9,10];
    let result:i64=binary_search::binary_search(&nums,7);
    assert_eq!(result, 6);    
}

pub fn binary_search_first_greater_than(){   
    let nums:Vec<i64>=vec![1,2,3,4,5,6,7,8,9,10];
    let result:i64=binary_search::binary_search_first_greater_than(&nums,7);
    assert_eq!(result, 7);    
}

pub fn binary_search_greater_or_equal_than(){   
    let mut nums:Vec<i64>=vec![1,2,3,4,5,6,7,8,9,10];
    let mut result:i64=binary_search::binary_search_greater_or_equal_than(&nums,7);
    assert_eq!(result, 6);    

    //One element greater case
    nums=vec![1];
    result=binary_search::binary_search_greater_or_equal_than(&nums,0);
    assert_eq!(result, 0); 
   //One element equal case
    nums=vec![1];
    result=binary_search::binary_search_greater_or_equal_than(&nums,1);
    assert_eq!(result, 0); 
    //One element non case
    nums=vec![1];
    result=binary_search::binary_search_greater_or_equal_than(&nums,2);
    assert_eq!(result, -1);   
}

pub fn binary_search_first_lower_than(){   
    let nums:Vec<i64>=vec![1,2,3,4,5,6,7,8,9,10];
    let result:i64=binary_search::binary_search_first_lower_than(&nums,7);
    assert_eq!(result, 5);    
}

pub fn binary_search_lower_or_equal_than(){   
    let mut nums:Vec<i64>=vec![1,2,3,4,5,6,7,8,9,10];
    let mut result:i64=binary_search::binary_search_lower_or_equal_than(&nums,7);
    assert_eq!(result, 6);    

    //One element lower case
    nums=vec![1];
    result=binary_search::binary_search_lower_or_equal_than(&nums,2);
    assert_eq!(result, 0); 
   //One element equal case
    nums=vec![1];
    result=binary_search::binary_search_lower_or_equal_than(&nums,1);
    assert_eq!(result, 0); 
    //One element non case
    nums=vec![1];
    result=binary_search::binary_search_lower_or_equal_than(&nums,0);
    assert_eq!(result, -1);   
}
