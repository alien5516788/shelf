/*
 * Clamps a name to a maximum length, truncating with ellipsis if necessary.
 */
pub fn clamp_name(name: &str, max_len: usize) -> String {
    let char_count = name.chars().count();

    if char_count <= max_len {
        name.to_string()  // Convert &str to String
    } else {
        let truncate_at = max_len.saturating_sub(3);
        name.chars().take(truncate_at).collect::<String>() + "..."
    }
}
