//! Manipulate with the `subject`.

/// Removes whitespaces from left and right sides of the `subject`.
///
/// # Arguments
///
/// * `subject` - The string to trim.
/// * `whitespace` - The whitespace characters to trim. List all characters that you want to be stripped.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::trim(" Mother nature ", "");
/// // => "Mother nature"
/// manipulate::trim("-~-Earth~-~", "-~");
/// // => "Earth"
/// ```
pub fn trim(subject: &str, whitespace: &str) -> String {
    trim_left_or_right(&subject, &whitespace, true, true)
}

/// Removes whitespaces from the left side of the `subject`.
///
/// # Arguments
///
/// * `subject` - The string to trim.
/// * `whitespace` - The whitespace characters to trim. List all characters that you want to be stripped.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::trim_left(" Mother nature ", "");
/// // => "Mother nature "
/// manipulate::trim_left("-~-Earth~-~", "-~");
/// // => "Earth~-~"
/// ```
pub fn trim_left(subject: &str, whitespace: &str) -> String {
    trim_left_or_right(&subject, &whitespace, true, false)
}

/// Removes whitespaces from the right side of the `subject`.
///
/// # Arguments
///
/// * `subject` - The string to trim.
/// * `whitespace` - The whitespace characters to trim. List all characters that you want to be stripped.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// manipulate::trim_right(" Mother nature ", "");
/// // => " Mother nature"
/// manipulate::trim_right("-~-Earth~-~", "-~");
/// // => "-~-Earth"
/// ```
// TODO: trim_left is deprecating in 1.33.0: superseded by trim_start
// TODO: trim_right is deprecating in 1.33.0: superseded by trim_end
// TODO: trim_left_matches is deprecating in 1.33.0: superseded by trim_start_matches
// TODO: trim_right_matches is deprecating in 1.33.0: superseded by trim_end_matches

pub fn trim_right(subject: &str, whitespace: &str) -> String {
    trim_left_or_right(&subject, &whitespace, false, true)
}

fn trim_left_or_right(subject: &str, whitespace: &str, to_left: bool, to_right: bool) -> String {
    if subject.len() == 0 {
        return subject.to_string();
    }
    if whitespace.len() == 0 {
        if to_left && to_right {
            return subject.trim().to_string();
        } else if to_left {
            return subject.trim_left().to_string();
        } else {
            return subject.trim_right().to_string();
        }
    }

    if to_left && to_right {
        return subject.trim_matches(|c| whitespace.contains(c)).to_owned();
    } else if to_left {
        return subject
            .trim_left_matches(|c| whitespace.contains(c))
            .to_owned();
    } else {
        return subject
            .trim_right_matches(|c| whitespace.contains(c))
            .to_owned();
    }
}