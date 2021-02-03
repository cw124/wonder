/// Helps write correct plurals.
///
/// # Examples
///
/// ```
/// assert_eq!(plural(0, "coin"), "0 coins");
/// assert_eq!(plural(1, "coin"), "1 coin");
/// assert_eq!(plural(2, "coin"), "2 coins");
/// ```
pub fn plural(count: i32, word: &str) -> String {
    if count == 1 {
        format!("{} {}", count, word)
    } else {
        format!("{} {}s", count, word)
    }
}
