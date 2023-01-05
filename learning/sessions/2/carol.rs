fn main() {
    let lyrics = [
        "and a Partridge in a Pear Tree",
        "2 Turtle Doves",
        "3 French Hens",
        "4 Calling Birds",
        "5 Golden Rings",
        "6 Geese a Laying",
        "7 Swans a Swimming",
        "8 Maids a Milking",
        "9 Ladies Dancing",
        "10 Lords a Leaping",
        "11 Pipers Piping",
        "12 Drummers Drumming"
    ];

    for i in 0..lyrics.len() {
        let suf = match i {
            0 => "st",
            1 => "nd",
            2 => "rd",
            _ => "th",
        };
        println!("On the {}{suf} day of Christmas", i+1);
        println!("my true love sent to me:");
        for j in 0..i+1 {
            if i == 0 {
                // let temp = lyrics[0];
                println!("A {}", &lyrics[0][6..]);
            } else {
                println!("{}", lyrics[i-j]);
            }
        }
    }
}