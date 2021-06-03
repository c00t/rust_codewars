use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        println!("{}",decode_bits("00000001100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011"));
        assert_eq!(false,true);
    }
}
// mod preloaded;
// use preloaded::MORSE_CODE;
// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".

// error-prone when input is "01110"
pub fn decode_bits(encoded: &str) -> String {
    // how to find the transsion rate?
    // but firstly trim begin and end
    let encoded = encoded.trim_matches('0');
    println!("{}",encoded);
    if !encoded.contains("0") {
        return ".".to_string();
    }
    let max_zeros = encoded.chars().filter(|c| *c=='0').count() as u32;
    // println!("{},{}",encoded,max_zeros);
    // use space to find the transsion rate
    // !!! make sure to evluate lazy expression.
    let zero_length:Vec<u32> = encoded.split('1').filter(|&x| {
        //println!("{}",x);
        x.len() != 0
    }).map(|x| x.len() as u32).collect();

    let one_length:Vec<u32> = encoded.split('0').filter(|&x| {
        //println!("{}",x);
        x.len() != 0
    }).map(|x| x.len() as u32).collect();
    //println!("{:?}",zero_length);
    let mut transmission_rate = 1;
    for i in 2..max_zeros+1 {
        if zero_length.iter().all(|&x| 
        {
            let n = x/i;
            let rem = x%i;
            rem == 0 && (n==1 || n==3 || n==7)
        }) && one_length.iter().all(|&x| 
        {
            let n = x/i;
            let rem = x%i;
            rem == 0 && (n==1 || n==3 || n==7)
        }){
            transmission_rate = i;
            break;
        }
    }
    // find the transmission_rate then
    // dot is '1'*rate
    // dash is '111'*rate
    // pause between dot&dash '0'*rate
    // pause between char '000'*rate
    // pause between words '0000000'*rate

    // it's dummy? no! the transimision may be 3 or 7 etc.
    // let dot = (0..transmission_rate).fold(String::from(""), |acc,x| acc + "1");
    // let dash = (0..transmission_rate).fold(String::from(""), |acc,x| acc + "111");
    // let pause_bt_dd = (0..transmission_rate).fold(String::from(""), |acc,x| acc + "0");
    // let pause_bt_ch = (0..transmission_rate).fold(String::from(""), |acc,x| acc + "000");
    // let pause_bt_wd = (0..transmission_rate).fold(String::from(""), |acc,x| acc + "0000000");
    //println!("{}",transmission_rate);
    let res :String = encoded.chars().step_by(transmission_rate as usize).collect();
    //println!("{}",res);
    res.split("0000000").map(|word| {
        word.split("000").map(|ch| {
            ch.split("0").map(|d2d| {
                if d2d == "1"{
                    "." // don't copy char from instructions
                }else{
                    "-"
                }
            }).collect::<Vec<&str>>().join("") // use replace() is more readrable
        }).collect::<Vec<String>>().join(" ")
    }).collect::<Vec<String>>().join("   ")
}

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

/* Example Code
pub fn decode_bits(encoded: &str) -> String {
    // Trim excess zeros at the start and end
    let encoded = encoded.trim_matches('0');
    
    // Get the length of a time unit by finding the shortest sequence of zeros or ones,
    // this will represent a time unit of one which equals a dot
    let rate = {
        let rate_ones = encoded
            .split("0")
            .filter_map(|ones| (!ones.is_empty()).then(|| ones.len()))
            .min()
            .unwrap_or(usize::MAX);
        let rate_zeros = encoded
            .split("1")
            .filter_map(|zeros| (!zeros.is_empty()).then(|| zeros.len()))
            .min()
            .unwrap_or(usize::MAX);
        rate_zeros.min(rate_ones)
    };

    // Parse the encoded message
    encoded
        .chars() // Iterate through the characters
        .step_by(rate) // Only parse every n-th code
        .collect::<String>() // Collect it into a string
        // Begin converting from 1/0 to dot/dash
        .replace("111", "-") // Dash
        .replace("1", ".") // Dot
        .replace("0000000", "   ") // Word seperator
        .replace("000", " ") // Letter seperator
        .replace("0", "") // Dot/Dash seperator
}

pub fn decode_morse(encoded: &str) -> String {
    encoded
        .trim()
        .split("   ")
        .map(|word| {
            word.split(" ")
                .filter_map(|letter| MORSE_CODE.get(letter).map(|letter| letter.clone()))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}
 */