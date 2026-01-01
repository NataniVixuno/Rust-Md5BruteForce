pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().rev().collect();
    
    for (i, &ch) in chars.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    
    result.chars().rev().collect()
}

pub fn format_float(f: f64) -> String {
    // Format with commas for the integer part and 2 decimal places
    let rounded = (f * 100.0).round() / 100.0;
    let integer_part = rounded as u64;
    let decimal_part = ((rounded - integer_part as f64) * 100.0).round() as u64;
    
    if decimal_part > 0 {
        format!("{}.{:02}", format_number(integer_part), decimal_part)
    } else {
        format_number(integer_part)
    }
}

