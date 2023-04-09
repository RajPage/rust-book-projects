fn main() {
    let gifts = [
        "A partridge in a pear tree",
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
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for i in 0..12 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            days[i]
        );
        for j in (0..i + 1).rev() {
            if i != 0 && j == 0 {
                println!("And {}", gifts[j])
            } else {
                println!("{}", gifts[j]);
            }
        }
        println!("");
    }
}
