pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first/* Step 1 */.collect::<String>() + c.as_str(),
    }
}

// Step 2.
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    return words.iter().map(|word | capitalize_first(word)).collect::<Vec<String>>();
}

// This function is only needed to test your code through main.rs
pub fn iterate_string_vec() {
     let words = vec!["hello", "world"];
     let capitalized_words: Vec<String> = vec![capitalize_first(words[0]), capitalize_first(words[1])];
     println!("{:?}", capitalized_words);
}

// Step 3.
pub fn capitalize_words_string(words: &[&str]) -> String {
    return words.iter().map(|word | c/* Something goes here */on is only needed to test your code through main.rs
pub fn iterate_into_string() {
     let words = vec!["hello", " ", "world"];
     let capitalized_words = w/* Something gos here */     println!("{:?}", capitalized_words);
}


