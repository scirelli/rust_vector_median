use std::io;
mod math;
mod company;

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
    let mut rtn: Vec<String> = Vec::new();
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    let punct = ['!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'];
    let mut buf:String;

    for mut p in String::from(s).split(" ") {
        let mut opt_char:Option<char> = None;
        buf = String::new();

        if p.ends_with(punct) {
            let mut chars = p.chars();
            opt_char = chars.next_back();
            p = chars.as_str();
        }

        if p.to_lowercase().starts_with(vowels) {
            buf = buf + p + "-hay";
        }else {
            buf = buf + &p[1..] + "-" + &p[0..1] + "ay";
        }
        if let Some(c) = opt_char {
            buf.push(c);
        }
        rtn.push(buf);
    }
    rtn.join(" ")
}

/**
 * Using a hash map and vectors, create a text interface to allow a user to add employee names
 * to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.”
 * Then let the user retrieve a list of all people in a department or all people in the company
 * by department, sorted alphabetically.
 */
fn start_company_db() -> () {
    parse_user_input();
}

fn parse_user_input() {
    let mut c = company::Company::new();

    println!("Modify or view database.");
    loop {
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");

        let p:Vec<&str> = inp.split(" ").collect();

        match p[0].to_lowercase().trim() {
            //Insert a person into a department
            "add" => {
                let k = p.get(3).unwrap_or(p.get(2).unwrap_or(p.get(1).unwrap_or(&"No Dept"))).trim().to_string();
                let u = p.get(1).unwrap_or(&"No user").trim();
                let l = c.database.entry(k.clone()).or_insert(Vec::new());
                println!("\tAdding {u} to {k}");
                l.push(u.to_string());
            },
            //List all people in the given department
            "list" =>{
                match p.get(1) {
                    Some(d) => {
                        let d = d.trim();
                        println!("Listing department '{d}'");
                        if let Some(v) = c.database.get(d) {
                            println!("\t{}", v.join(", "));
                        }
                    },
                    None => println!("You must provide a department to list.")
                };
            },
            "exit" => {
                println!("Exiting application.");
                break;
            },
            //Dump the DB
            _ => {
                println!("Dumping the database");
                for (key, value) in &c.database {
                    println!("Department {key}:");
                    println!("\t{}", value.join(", "));
                }
            }
        };
    }
}

fn main() {
    test_median();
    println!("{}", to_pig_latin(&"Steve speaks latin. I do not.".to_string()));
    start_company_db();
}
