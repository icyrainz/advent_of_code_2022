pub fn run(lines: &Vec<String>) {
    
    let get_point = |c| match c {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Not supported action!"),
    };
    
    let compare = |(left, right): (&char, &char)| match (*left, *right) {
        ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
        ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6, 
        ('A', 'Z') | ('B', 'X') | ('C', 'Y') => 0,
        _ => panic!("Not supported action!"),
    };
    
    let mut score: u32 = 0;
    for line in lines {
        let first_action = line.chars().nth(0).unwrap();
        let second_action = line.chars().nth(2).unwrap();
        
        score += compare((&first_action, &second_action)) + get_point(second_action);
    }
    println!("score: {}", score);
}