

/// Capitalize first letter of each word.
pub fn capitalize(s: &str) -> String {
    return s.split(' ').map(|word| {
        match word.len() {
            0 => "".to_string(),
            1 => word.to_uppercase().to_string(),
            _ => word.chars().nth(0).unwrap().to_uppercase().to_string() + word.chars().skip(1).collect::<String>().as_str(),
        }
    }).reduce(|mut acc, word| {
        acc.push_str(" ");
        acc.push_str(word.as_str());
        acc
    }).unwrap();
    // let mut prev_is_space = true;
    //
    // let mut new_s = String::new();
    // for ch in s.chars() {
    //     match ch {
    //         ' ' => {
    //             prev_is_space = true;
    //             new_s.push(ch);
    //         },
    //         _ => {
    //             if prev_is_space {
    //                 new_s += ch.to_uppercase().to_string().as_str();
    //             } else {
    //                 new_s.push(ch);
    //             }
    //             prev_is_space = false;
    //         }
    //     }
    // }
    // new_s
}
