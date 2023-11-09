
#![allow(dead_code)]
use std::fmt::Debug; 

/***
 * First let's decide what's the api for array_list 
 * 
 * let mut arr = ArrayList::<i32>::new(); 
 * arr.append(1); 
 * arr.append(2); 
 * arr.append(3); 
 * arr.append(4); 
 * arr.append(5); 
 * arr.pop(); //should return 5 
 * arr.get(0); // should return the 0th element in the current list 
 * arr.remove(3); // should remove 3 from this list 
 * arr.remove_at(0); // removes the 0th element in the current list 
 * 
 */

#[derive(Debug)]
pub struct ArrayList<T> {
    pub len: usize, // tracks the number of elements present
    inner: Vec<T>,
} 

impl<T: Debug+ Default+ Clone+ PartialEq> ArrayList<T> {

    pub fn new() -> ArrayList<T> {
        ArrayList{
            len: 0, 
            inner: Vec::<T>::new(),
        }
    }

    fn expand_inner(&mut self) {
        // initialize an array of size 2*len 

        let mut arr2 = vec![T::default(); 2*self.len +1 as usize]; 
        
        for i in 0..self.len{
            arr2[i] = self.inner[i].clone(); 
        }

        self.inner = arr2; 
    
    }

    pub fn append(&mut self, item: T) {

        if self.len == self.inner.len() {
            // inner vec has reached it's capacity 
            self.expand_inner();
        }
        self.inner[self.len] = item; 
        // why is append showing error 
        // self.inner.append(item); 
        self.len += 1; 
    }

    pub fn pop(&mut self) -> Option<T> {

        if self.len == 0{
            return None;
        }

        // here we are cloning and returning what if we don't want to clone ? 
        let ret = self.inner[self.len].clone(); 
        self.len-=1; 

        return Some(ret); 

    }

    pub fn get(&self, index: usize ) -> Option<&T> {

        if index > self.len {
            return None;
        }

        return Some(&self.inner[index]);
    }

    pub fn remove(&mut self, item: &T) -> Option<T>{
        for i in 0..self.inner.len(){

            if &self.inner[i] == item {
                return self.remove_at(i);
            } 
        }
        return None;

    }

    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        
        if index > self.len {
            return None;
        }

        let ret = self.inner[index].clone(); 

        for i in index..self.len {
            self.inner[i] = self.inner[i+1].clone(); 
        }

        self.len -=1; 
        return Some(ret);
    
    }

    pub fn insert_at(&mut self, index: usize, item: T) {


        if self.len == self.inner.len() {
            self.expand_inner();
        }

        for i in ((index+1)..self.len).rev() {
            self.inner[i] = self.inner[i-1].clone(); 
        }

        self.inner[index] = item.clone(); 
        
        self.len += 1;
    }



}




// Having hard time asking cargo to run this specifically 
#[cfg(tests)]
mod array_list{

    use super::*; 

    #[test]
    fn test1(){
        println!("Test 1");
        
        let mut arrList = ArrayList::<i32>::new();

        arrList.append(4); 
        arrList.append(2); 
        arrList.append(5); 
        arrList.append(2); 
        arrList.append(-24);

        assert_eq!(4, arrList.get(0)); 

    }
}