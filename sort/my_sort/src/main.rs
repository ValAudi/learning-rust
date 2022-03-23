fn my_sort(arr: &mut [i32]) {
    let len = arr.len();
    for counter in 0..len {
        for inner_counter in 0..len {
            println!("{}", arr[counter]);
            println!("Before swap {:?}", arr);
            println!("We are at {} {}", counter, inner_counter);
            if arr[counter] > arr[inner_counter] {
                arr.swap(counter, inner_counter);
                // println!("After Swap {:?}", arr);
            }
            println!("After swap{:?}", arr);
        }
        println!("End of loop {}", counter)
    }
    println!("{:?}", arr)
}

fn main() {
    let mut arr = vec![3,5,7,0,2,1];
    my_sort(&mut arr);
}
