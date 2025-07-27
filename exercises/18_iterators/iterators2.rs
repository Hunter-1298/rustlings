// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// // ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // ???
    let mut res: Vec<String> = vec![];
    for word in words {
        res.push(capitalize_first(word));
    }
    res
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    // ???
    let mut res = String::new();
    for word_slice in words {
        res.push_str(capitalize_first(word_slice).as_str());
    }
    res
}

fn main() {
    // You can optionally experiment here.
    let test = "testing";
    let upper = capitalize_first(test);
    println!("{}", upper);

    let words = vec!["hello", "world"];
    let upper_word = capitalize_words_vector(&words);
    for word in upper_word {
        println! {"{}", word};
    }

    let words = vec!["hello", " ", "world"];
    let upper_words = capitalize_words_string(&words);
    println!("{}", upper_words);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
