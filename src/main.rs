fn main() {
    let lyrics = [
        "And a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh",
        "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    for i in 0..12 {
        println!(
            "On the {} day of Christmas my true Love gave to me",
            days[i]
        );
        if i == 0 {
            println!("A partridge in a pear tree");
        } else {
            for n in (0..i + 1).rev() {
                println!("{}", lyrics[n])
            }
        }
        println!("")
    }
}
