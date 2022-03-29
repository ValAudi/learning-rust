fn main() {
    let list1: Vec<i32> = vec![3, 5, 9, 10, 12, 20];
    let list2: Vec<i32> = vec![1, 4, 14, 18];
    let group_median: f64 = merge_array_median(list1, list2);
    println!("{}", group_median);

}

fn merge_array_median(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
    let mut merged_arr: Vec<i32> = Vec::new();
    if nums1.len() > nums2.len() {
        let mut nums2_iter = nums2.iter();
        while nums2_iter.next() != None {
            let merged_arr = binary_search_insert(&mut nums1, &nums2_iter.next().unwrap());    
        }
    }
    else {
        let mut nums1_iter = nums1.iter();
        while nums1_iter.next() != None {
            let merged_arr = binary_search_insert(&mut nums2, &nums1_iter.next().unwrap());
        }
    }
    if &merged_arr.len() % 2 == 0 {
        let middle_value1: &i32 = &merged_arr.get(&merged_arr.len()/2).unwrap();
        let middle_value2: &i32 = &merged_arr.get((&merged_arr.len()/2) - 1).unwrap();
        let median_value: f64 = ((middle_value1 + middle_value2)/2) as f64;
        return median_value;
    }
    else {
        let middle_value = *(&merged_arr.get(&merged_arr.len()/2).unwrap());
        let median_value: f64 = *middle_value as f64;
        return median_value;
    }
}

fn binary_search_insert<'a>(start_arr: &'a mut Vec<i32>, insert_value: &'a i32) -> &'a Vec<i32> {
    let mid_point: usize = &start_arr.len()/2;
    if insert_value > &(start_arr[mid_point] as i32) {
        if [mid_point..].len() > 1 { 
            let mut inter_arr = &start_arr[mid_point..];
            let mut inter_vec = inter_arr.to_vec();
            binary_search_insert(&mut inter_vec, &insert_value);
        } else {
            start_arr.insert(0, *insert_value); 
        }; 
    } else {
        if [..mid_point].len() > 1 { 
            let mut inter_arr = &start_arr[..mid_point];
            let mut inter_vec = inter_arr.to_vec();
            binary_search_insert(&mut inter_vec, &insert_value);
        } else {
            start_arr.insert(1, *insert_value); 
        };
    }
    return start_arr;
}
