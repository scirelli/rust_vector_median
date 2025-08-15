mod math;

/**
 * Given a list of integers, use a vector and return the median (when sorted, the value in the
 * middle position) and mode (the value that occurs most often; a hash map will be helpful here) of
 * the list.
 */
fn test_median() {
    let mut arr:[u8; 10] = [0; 10];
    rand::fill(&mut arr[..]);
    let mut v = Vec::from(arr);
    println!("Arr: {arr:?}");
    println!("Vec: {v:?}");
    arr[0] = 34;
    println!("Modified arr: {arr:?}");
    println!("Vec: {v:?}");
    let k = math::median(&mut v);
    println!("Median: {k:?}");
    println!("Vec: {v:?}");
}

/**
 * Convert strings to pig latin. The first consonant of each word is moved to the end of the word
 * and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the
 * end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
 */
fn to_pig_latin(s: &str) -> String {
    let mut rtn = String::new();
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];

    for p in String::from(s).split(" ") {
        if p.starts_with(vowels) {
            rtn = rtn + p + " -fay ";
        }else {
            rtn = rtn + &p[1..] + "-" + &p[0..1] + "ay";
        }
    }
    rtn
}

fn main(){
    test_median();
    println!("{}", to_pig_latin(&"a steve speaks latin".to_string()));
}
