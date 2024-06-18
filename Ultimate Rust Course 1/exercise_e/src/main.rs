#![allow(unused_mut, unused_variables)]

fn main() {
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });
    
    fn inspect(s: &String) {
        if s.ends_with("s") {
            println!("plural");
        } else {
            println!("singular");
        }
    }
    inspect(&arg);

    fn change(s: &mut String) {
        if !s.ends_with("s") {
            s.push_str("s")
        }    }

    change(&mut arg);
    println!("I have many {}", arg);

    fn eat(s: String) -> bool {
        if s.starts_with("b") && s.contains("a") {
            true
        } else {
            false
        }
    }

    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }

    fn bedazzle(s: &mut String) -> String {
        *s = String::from("sparkly");
        s.to_string()
    }

    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}
