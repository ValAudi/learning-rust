fn main() {
    // Creating a frequency HashMap from a Vector array
    let map <i32, i32> = HashMap::new();
    for key in array_play {
        let count = map.entry(key).or_insert(0);
        *count =+ 1
    }
    println!("{:?}", map);


    // builder a HashMap with Vec 
    let mut mapper: HashMap<i32, Vec<&str>> = HashMap::from([(2, vec!["a", "b", "c"]), 
                                                             (3, vec!["d", "e", "f"]),
                                                             (4, vec!["g", "h", "i"]),
                                                             (5, vec!["j", "k", "l"]),
                                                             (6, vec!["m", "n", "o"]),
                                                             (7, vec!["p", "q", "r", "s"]),
                                                             (8, vec!["t", "u", "v"]),
                                                             (9, vec!["w", "x", "y", "z"])]);
}
