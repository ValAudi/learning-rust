fn main() {
    let list1: Vec<i32> = vec![3, 5, 9, 10, 12, 20];
    let list2: Vec<i32> = vec![1, 4, 14, 18];
    //let group_median: f64 = merge_array_median(list1, list2);
    // println!("{}", group_median);

}

fn merge_array_median(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut merged_arr: Vec<i32> = Vec::new();
    if nums1.len() > nums2.len() {
        let nums2_iter = nums2.iter();
        while nums2_iter.next() =! None {
            &merged_arr = binary_search_insert(&nums1, &nums2_iter.next().unwrap());    
        }
    }
    else {
        let nums1_iter = nums1.iter();
        while nums1_iter.next() != None {
            &merged_arr = binary_search_insert(&nums2, &nums1_iter.next().unwrap());
        }
    }
    if &merged_array.len() % 2 == 0 {
        let median_value: f64 = (&merged_array.get(&merged_array.len()/2) + 
                                  &merged_array.get((&merged_array.len()/2) - 1))
                                  /2;
        return median_value;
    }
    else {
        let median_value: f64 = &merged_array.get(&merged_array.len()/2);
        return median_value;
    }
}

fn binary_search_insert<'a>(start_arr: &'a Vec<i32>, insert_value: &'a i32) -> &'a Vec<i32> {
    let mid_point: i32 = (start_arr.len()/2).try_into().unwrap();
    if insert_value > start_arr[mid_point] as &i32 {
        if [mid_point..].len() > 1 { 
            let mut inter_arr = [mid_point..];
            binary_search_insert(&inter_arr, &insert_value);
        } else {
            start_arr.insert(0, *insert_value); 
        }; 
    } else {
        if [..mid_point].len() > 1 { 
            let mut inter_arr = [..mid_point];
            binary_search_insert(&inter_arr, &insert_value);
        } else {
            start_arr.insert(1, *insert_value); 
        };
    }
    return start_arr;
}
