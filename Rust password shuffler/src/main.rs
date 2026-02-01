use rand::rngs::OsRng;
use rand::seq::SliceRandom;

fn generate_password(length: usize) -> String {
    let lower = "abcdefghijklmnopqrstuvwxyz";
    let upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let digits = "0123456789";
    let symbols = "!@#$%^&*()-_=+[]{};:,.<>?/|~";

    
    let hindi = (0x0904..=0x0939)
        .filter_map(std::char::from_u32)
        .collect::<Vec<char>>();

    let chinese = (0x4E00..=0x9FFF)
        .filter_map(std::char::from_u32)
        .collect::<Vec<char>>();

    let hiragana = (0x3041..=0x3096)
        .filter_map(std::char::from_u32)
        .collect::<Vec<char>>();

    let katakana = (0x30A1..=0x30FA)
        .filter_map(std::char::from_u32)
        .collect::<Vec<char>>();

    let korean = (0xAC00..=0xD7A3)
        .filter_map(std::char::from_u32)
        .collect::<Vec<char>>();

    let cyrillic = (0x0410..=0x044F)
        .filter_map(std::char::from_u32)
        .collect::<Vec<char>>();

    let mut password: Vec<char> = Vec::new();

    
    password.push(*lower.chars().collect::<Vec<_>>().choose(&mut OsRng).unwrap());
    password.push(*upper.chars().collect::<Vec<_>>().choose(&mut OsRng).unwrap());
    password.push(*digits.chars().collect::<Vec<_>>().choose(&mut OsRng).unwrap());
    password.push(*symbols.chars().collect::<Vec<_>>().choose(&mut OsRng).unwrap());

    
    password.push(*hindi.choose(&mut OsRng).unwrap());
    password.push(*chinese.choose(&mut OsRng).unwrap());
    password.push(*korean.choose(&mut OsRng).unwrap());
    password.push(*cyrillic.choose(&mut OsRng).unwrap());

    
    let mut ascii_pool: Vec<char> = Vec::new();
    ascii_pool.extend(lower.chars());
    ascii_pool.extend(upper.chars());
    ascii_pool.extend(digits.chars());
    ascii_pool.extend(symbols.chars());

    let mut unicode_pool: Vec<char> = Vec::new();
    unicode_pool.extend(hindi.iter());
    unicode_pool.extend(chinese.iter());
    unicode_pool.extend(hiragana.iter());
    unicode_pool.extend(katakana.iter());
    unicode_pool.extend(korean.iter());
    unicode_pool.extend(cyrillic.iter());

    // fill rest (80% ascii, 20% unicode)
    while password.len() < length {
        if rand::random::<f32>() < 0.8 {
            password.push(*ascii_pool.choose(&mut OsRng).unwrap());
        } else {
            password.push(*unicode_pool.choose(&mut OsRng).unwrap());
        }
    }

    password.shuffle(&mut OsRng);
    password.into_iter().collect()
}

fn main() {
    // terminal colors (works on macOS Terminal I think...)
    let blue = "\x1b[34m";
    let orange = "\x1b[38;5;208m";
    let green = "\x1b[32m";
    let gray = "\x1b[90m";
    let pink = "\x1b[35m"; 
    let reset = "\x1b[0m";

    // I used patorjk.com/software/taag/ for this...
    println!(
        r#"
    ________  ________  ________  ________  ________  ________  ________   _______       _______  ________  ________  ________  ________  ________  ________  ________ 
  /        \/        \/        \/        \/  /  /  \/        \/        \_/       \    //       \/        \/    /   \/        \/        \/        \/        \/        \
 /         /         /        _/        _/         /         /         /         /   //      __/         /         /         /         /        _/         /         /
//      __/         /-        /-        /         /         /        _/         /   /       / /        _/         /        _/         //       //         /        _/ 
\\_____/  \___/____/\________/\________/\________/\________/\____/___/\________/    \________/\________/\__/_____/\____/___/\___/____/ \______/ \________/\____/___/                                                                                                                                                                                                 
                                                                                         
"#
    );

    
    // F warning block
    println!("{}Warning:{}",
        orange, reset
    );
    println!("{}- Store generated passwords in a password manager (PLEASE).{}",
        green, reset
    );
    println!("{}- Do not reuse passwords across multiple services (PLEASEEEE).{}",
        green, reset
    );
    println!("{}- This app does not store any generated data.{}",
        green, reset
    );
    println!("{}  (Check the code yourself if you don't believe me.){}\n",
        gray, reset
    );

    // Passwords
    let count = 6;
    let length = 30;

    for i in 1..=count {
        let pw = generate_password(length);
        println!("{}Password {}:{} {}", pink, i, reset, pw);
    }
    println!("\nMade by YaYaYanKh on Twitter");
}
