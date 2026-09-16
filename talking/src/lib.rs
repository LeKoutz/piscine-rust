pub fn talking(text: &str) -> &str {
    let text = text.trim();
    let is_yelling = text.chars().any(|c| c.is_alphabetic()) && text.chars().all(|c| !c.is_lowercase());
    let is_question = text.trim().ends_with("?");

    match (text.is_empty(), is_yelling, is_question) {
        (true, _, _) => "Just say something!",
        (_, true, true) => "Quiet, I am thinking!",
        (_, true, false) => "There is no need to yell, calm down!",
        (_, false, true) => "Sure.",
        (_, false, false) => "Interesting",
    }
}
