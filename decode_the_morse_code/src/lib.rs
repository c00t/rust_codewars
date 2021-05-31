use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".

// a simple morse decoder
fn decode_morse(encoded: &str) -> String {
    // comment when submit
    let MORSE_CODE:HashMap<String,String> = HashMap::new();
    
    let encoded = encoded.trim();
    encoded.split("   ").map(|x| {
        //words here
        x.split(" ").map(|word| {
            if let Some(w) = MORSE_CODE.get(word) {
                w
            }else{
                ""
            }
        }).collect::<Vec<&str>>().join("")
    }).collect::<Vec<String>>().join(" ")
}