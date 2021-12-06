/**
    Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/
pub fn gauss(n: i32) -> i32 {
    
    if n < 0 {
        return -1;
    } else {
        let mut curr = 0;
        for i in 1..=n {
            curr = curr + i;
            
        }
        return curr;
    }
}

/**
    Returns the number of elements in the list that 
    are in the range [s,e]
**/
pub fn in_range(ls: &[i32], s: i32, e: i32) -> i32 {
    
    let mut counter = 0;

    for num in ls {
        if num >= &s && num <= &e {
            counter = counter + 1;
        }
    }

    return counter;

}

/**
    Returns true if target is a subset of set, false otherwise

    Ex: [1,3,2] is a subset of [1,2,3,4,5]
**/
pub fn subset<T: PartialEq>(set: &[T], target: &[T]) -> bool {
    
    // For each element of the set, if it is not an element of the target return false
    for target_elem in target {
        if set.contains(target_elem) == false { // MAKE SURE WE ASK IF THIS IS OKAY ****
            return false;
        }
    }

    return true;

}

/**
    Returns the mean of elements in ls. If the list is empty, return None
    It might be helpful to use the fold method of the Iterator trait
**/
pub fn mean(ls: &[f64]) -> Option<f64> {
    
    let sum = ls.iter().fold(0 as f64, |acc, &x| acc + x);
    let len = ls.len() as f64;
    
    if len == 0 as f64 {
        return None;
    }

    return Some (sum / len);

}

/**
    Converts a binary number to decimal, where each bit is stored in order in the array
    
    Ex: to_decimal of [1,0,1,0] returns 10
**/
pub fn to_decimal(ls: &[i32]) -> i32 {
    // For each bit in the array we multiply the accumulator by two, because if we managed to go one place more
    // Then the previous bits were worth twice as much as we thought they were, then add the current bit
    ls.iter().fold(0, |acc, &b| acc*2 + b)
}

/**
    Decomposes an integer into its prime factors and returns them in a vector
    You can assume factorize will never be passed anything less than 2

    Ex: factorize of 36 should return [2,2,3,3] since 36 = 2 * 2 * 3 * 3
**/
pub fn factorize(n: u32) -> Vec<u32> {
    unimplemented!()
    // Probably going to be recursive factoring tree method with a prime check helper function
    // GCD?
}

/** 
    Takes all of the elements of the given slice and creates a new vector.
    The new vector takes all the elements of the original and rotates them, 
    so the first becomes the last, the second becomes first, and so on.
    
    EX: rotate [1,2,3,4] returns [2,3,4,1]
**/
pub fn rotate(lst: &[i32]) -> Vec<i32> {
    
    let len = lst.len();
    let mut vec = Vec::new();

    // If the slice is empty return an empty vec
    if len == (0 as usize) {
        return vec;
    }

    // Only look for the next elements if there is more than one in the slice
    if len > 1 as usize {
        for i in 1..len {
            vec.push(lst[i]);
            
        }
    }

    vec.push(lst[0 as usize]);

    return vec;

}

/**
    Returns true if target is a subtring of s, false otherwise
    You should not use the contains function of the string library in your implementation
    
    Ex: "ace" is a substring of "rustacean"
**/
pub fn substr(s: &String, target: &str) -> bool {
    
    let tar_len = target.len();
    let s_len = s.len();

    if target == "" {
        return true;
    }

    // For each character in s, check to see if the next tar_len characters match the target
    for i in 0..s_len {
        if (s_len - i) < tar_len {
            return false;
        } 
        if &s[i..i+tar_len] == target {
            return true;
        }
    }

    return false;

}

/**
    Takes a string and returns the first longest substring of consecutive equal characters

    EX: longest_sequence of "ababbba" is Some("bbb")
    EX: longest_sequence of "aaabbb" is Some("aaa")
    EX: longest_sequence of "xyz" is Some("x")
    EX: longest_sequence of "" is None
**/
pub fn longest_sequence(s: &str) -> Option<&str> {

    let s_len = s.len();
    let mut start = 0;
    let mut end = 0;
    let mut temp_start = 0;
    let mut temp_end = 0;
    let mut count = 0;
    let mut temp_count = 0;
    let mut curr_char = "";

    if s == "" {
        return None;
    }

    // For each character in the string
    for i in 0..s_len {

        // If the current character is part of the currently running sequence
        if curr_char == &s[i..=i] {
            temp_count = temp_count + 1; // Increment the temp_count counter and the temporary end pointer
            temp_end = temp_end + 1;

            if temp_count > count { // If the current sequence gets to be longer than the old one
                count = temp_count; // Set count to the new longest count
                end = temp_end; // Set end to the new end of sequence index
                start = temp_start; // Set start to the new start of sequence index
            }

        } else { // If the current character is NOT part of the currently running sequence
            curr_char = &s[i..=i]; // Set the current seq char to the new character
            temp_start = i; // temp start is the index of the change
            temp_end = i; // temp end is also the index of the change
            temp_count = 1; // Set temp count to 1 because we have seen one of this character
        }

    }

    return Some (&s[start..=end]);
}
