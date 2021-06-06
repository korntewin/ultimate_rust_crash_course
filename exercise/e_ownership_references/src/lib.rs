pub fn inspect(word: &String) {

    if word.ends_with("s") {println!("Plural")}
    else {println!("Singular")}

}

pub fn change(word: &mut String) {

    if !word.ends_with("s") {word.push_str("s")}

}

pub fn eat(word: String) -> bool {

    if word.starts_with("b") & word.contains("a") {
        return true
    }
    
    return false
}

pub fn add(n1: &i32, n2: &i32) -> i32 {
    (*n1) + (*n2)
}

