fn main() {
    let list1: Vec<i32> = vec![3, 5, 9, 10, 12, 20];
    let list2: Vec<i32> = vec![1, 4, 14, 18];
    let grouped_arr = merged_array(list1, list2);
    println!("{:?}", grouped_arr);
}

fn merged_array(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32>{
    if nums1.len() > nums2.len(){
        nums1.sort();
        let mut nums2_iter = nums2.iter();
        for _i in 0..nums2.len() {
            let num_to_insert = nums2_iter.next().unwrap();
            let insert_index = |num_to_insert| nums1.binary_search(&num_to_insert).unwrap_or_else(|x| x);
            nums1.insert(insert_index(*num_to_insert), *num_to_insert);
        }
        return  nums1;        
    } else {
        nums2.sort();
        let mut nums1_iter = nums1.iter();
        for _i in 0..nums1.len() {
            let num_to_insert = nums1_iter.next().unwrap();
            let insert_index = |num_to_insert| nums2.binary_search(&num_to_insert).unwrap_or_else(|x| x);
            nums2.insert(insert_index(*num_to_insert), *num_to_insert);
        } 
        return nums2;
    }
    
    
}
