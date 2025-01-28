pub fn find_all_occurrences(input: &str, looking_for: &str) -> Vec<usize> {
    let mut indexes: Vec<usize> = Vec::new();
    let mut remover = input;
    let mut index_shift_val = 0;
    loop {
        if let Some(v) = remover.find(looking_for) {
            indexes.push(v + index_shift_val);
            index_shift_val += v+1;
            remover = &remover[(v+looking_for.len())..];
        }
        else {
            break;
        }
    }

    indexes
}

pub fn find_nth_occurrence(input: &str, looking_for: &str, n: u32) -> Option<usize> {
    let mut exhauster = input;
    let mut erased_count = 0;
    let mut i: u32 = 0;

    loop {

        println!(r#"This occurrence: "{exhauster}""#);

        if let Some(v) = exhauster.find(looking_for) {
            println!("Found match for this occurrence at {v}");
            if i == n {
                return Some(v+erased_count);
            }

            let shorten_beginning_by = v + looking_for.len();
            erased_count += shorten_beginning_by;
            exhauster = &exhauster[shorten_beginning_by..];
            i += 1;
            // otherwise if its not a match, shorten the n
        }
        else {
            return None; // break out of the function and dw about the loop at all
        }
    }
}

pub fn find_nth_occurrence_bounds(input: &str, looking_for: &str, n: u32) -> Option<(usize, usize)> {
    let starting_index: Option<usize> = find_nth_occurrence(input, looking_for, n);
    match starting_index {
        Some(v) => {
            Some((v, v+looking_for.len()))
        }
        None => {
            None
        }
    }
}

pub fn get_string_in_bounds(input: &str, bounds: (usize, usize)) -> Result<&str, &str> {
    // check for bound validity
    if bounds.1 < bounds.0 {
        return Err("Error: the first bound cannot be bigger");
    }
    // check if the end index fits into the string slice
    if input.len() < bounds.1 {
        return Err("Error: the end index is out of bounds");
    }

    Ok(&input[bounds.0..bounds.1])
}
