pub fn abbreviate(phrase: &str) -> String {
    //todo!("Given the phrase '{phrase}', return its acronym");
    let mut acronym = String::new();
    let skip = [' ', '-', '_'];
    let mut prev = ' ';

    for c in phrase.chars() {
        if c.is_alphabetic() && skip.contains(&prev) {
            acronym.push(c);
        } else if c.is_uppercase() && prev.is_lowercase() {
            acronym.push(c);
        }

        prev = c;
    }

    acronym.to_uppercase()

}
