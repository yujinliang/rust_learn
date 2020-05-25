mod error;
mod count;
pub use error::WordCountError;
pub use count::count_words;

#[cfg(test)]
mod tests {
    use crate::count::count_words;

    #[test]
fn count_words_t() {
        let mut b = "This string will be read".as_bytes();
        let r = count_words(& mut b);
        assert!(r.is_ok()); 
        assert!(r.unwrap() == 5);
    }
}

