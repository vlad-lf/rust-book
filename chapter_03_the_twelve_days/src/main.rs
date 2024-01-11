const NUMERICALS: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "nineth",
    "tenth",
    "eleventh",
    "twelfth"
];

const PRESENTS: [&str; 12] = [
    "Twelve drummers drumming",
    "Eleven pipers piping",
    "Ten lords a-leaping",
    "Nine ladies dancing",
    "Eight maids a-milking",
    "Seven swans a-swimming",
    "Six geese a-laying",
    "Five golden rings",
    "Four calling birds",
    "Three French hens",
    "Two turtle doves",
    "And a partridge in a pear tree"
];

const SECOND_LINE: &str = "my true love gave to me";


fn main() {
    println!("On the {} day of Christmas, \n{SECOND_LINE}\nA partridge in a pear tree.\n", NUMERICALS[0]);

    let mut index = 1;

    while index < 12 {
        println!("On the {} day of Christmas, \n{SECOND_LINE}", NUMERICALS[index]);

        for i in 12 - index - 1 .. 12 {
            println!("{}", PRESENTS[i]);
        }

        println!("");

        index += 1;
    }
}
