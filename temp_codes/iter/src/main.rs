use std::collection::HashMap;

fn main() {
    let mut array_play = vec![9, 5, 8, 3, 7, 1, 4, 2, 0];
    println!("{:?}", array_play);


    // Get the length of a array
    let length = array_play.len();
    println!("{}", length);
    
    //Get count of all elements in an array
    let count_elem = array_play.iter().count();
    println!("{:?}", count_elem);


    // Get sum of all elements in array
    let sum_array: i32 = array_play.iter().sum();
    println!("{:?}", sum_array);


    //Get max value of elements in array
    let max_value = array_play.iter().max().unwrap();
    println!("{:?}", max_value);


    //get min value of elemtns in array
    let min_value = array_play.iter().min().unwrap();
    println!("{:?}", min_value);


    //Get the position of an element in an array
    let value_index = array_play.iter().position(|&x| x == 1).unwrap();
    println!("{:?}", value_index);


    // Get how many times an element appears in a vector
    let elem_count = array_play.iter().filter(|&x| x == i).count();
    println!("{:?}", elem_count);

    // Get the reverse of an array and form a new array
    let reverse_array = array_play.iter().rev().collect();
    println!("{:?}", reverse_array);

    
    // Get slices of an array
    let array_slice = array_play.get(1..8).unwrap();
    println!("{:?}", array_slice);
    // OR
    let new_slice = &array_play[1..8];
    println!("{:?}", new_slice);

    // how to check if an element is contained in an array
    let boo = array_play.contains(&9);
    println!("{}", boo);


