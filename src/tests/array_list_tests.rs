use crate::{*, array_list::ArrayList}; // import everything 

#[test]
fn test1(){
    println!("Test 1");
    
    let mut arrList = ArrayList::<i32>::new();

    arrList.append(4); 
    arrList.append(2); 
    arrList.append(5); 
    arrList.append(2); 
    arrList.append(-24);

    print!("{:?}",arrList);
    assert_eq!(Some(&4), arrList.get(0)); 

}