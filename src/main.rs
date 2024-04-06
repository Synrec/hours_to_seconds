use std::io;

fn main (){
    println!("Input number of hours to convert to string:");
    let mut hours = String::new();
    io::stdin()
        .read_line(&mut hours)
        .expect("Failed to read.");
    let hour_num: u32 = match hours.trim().replace(" ", "").parse(){
        Ok(num) => num,
        Err(_) => return ()
    };
    let hours_in_seconds:String = (60 * 60 * hour_num).to_string();
    println!("{hours_in_seconds} seconds in {hour_num} hours.");
}