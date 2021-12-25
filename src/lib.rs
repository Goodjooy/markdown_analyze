use std::collections::HashSet;

mod marco;
mod  interface;
mod utils;

mod dfa;
mod nfa;

#[test]
fn test_set() {
    let a =HashSet::<usize>::from_iter(vec![1,2,3,4,5].into_iter());
    let b=HashSet::<usize>::from_iter(vec![1,2].into_iter());
    
    let c= a.intersection(&b);

    println!("{:?}",c);
}