pub fn split_string(input: &String) -> Vec<String> {
    let mut components = Vec::new();
    let mut current_component = String::new();
    let mut is_negative = false;

    for c in input.chars() {
        if c == '_' {
            components.push(if is_negative {
                "-".to_string() + &current_component
            } else {
                current_component.clone()
            });
            current_component.clear();
            is_negative = false;
        } else if c == '-' {
            is_negative = true;
        } else {
            current_component.push(c);
        }
    }

    components.push(if is_negative {
        "-".to_string() + &current_component
    } else {
        current_component
    });

    components
}

pub fn parse_segment(segment: &str) -> (i32, i32) {
    let strand = if segment.starts_with("-") {
        segment.chars().skip(2).collect::<String>()
    } else {
        segment.chars().skip(1).collect::<String>()
    };

    let mut under = strand
        .parse::<i32>()
        .expect("Failed to parse strand number");
    under -= 1; // reindex
    let over: i32;
    if segment.starts_with("-") {
        over = under;
        under += 1
    } else {
        over = under + 1
    }

    (over, under)
}
