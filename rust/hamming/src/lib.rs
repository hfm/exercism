pub fn hamming_distance(a: &str, b: &str) -> Result<i8, bool> {
    if a.len() != b.len() {
        return Err(false);
    }

    let mut count = 0;
    for (i, _) in a.char_indices() {
        if a.chars().nth(i) != b.chars().nth(i) {
            count += 1;
        }
    }

    return Ok(count);
}
