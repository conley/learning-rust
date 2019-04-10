const ARTICLES: [&str; 12] = [
    "a",
    "two",
    "three",
    "four",
    "FIVE...",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
];

fn article(day: usize) -> String {
    let idx: usize = day - 1;
    ARTICLES[idx].to_string()
}

const GIFTS: [&str; 12] = [
    "partridge in a pear tree!",
    "turtle doves, and",
    "french hens,",
    "calling birds,",
    "GOLDEN RINGS,",
    "geese a-laying,",
    "swans a-swimming,",
    "maids a-milking,",
    "ladies dancing,",
    "lords a-leaping,",
    "pipers piping,",
    "drummers drumming,",
];

fn gift(day: usize) -> String {
    let idx: usize = day - 1;
    GIFTS[idx].to_string()
}

fn ordinal(day: usize) -> String {

    String::from(if day == 1 {
        "first".to_string()
    } else if day == 2 {
        "second".to_string()
    } else if day == 3 {
        "third".to_string()
    } else if day == 5 {
        "fifth".to_string()
    } else if day == 9 {
        "ninth".to_string()
    } else if day == 12 {
        "twelfth".to_string()
    } else {
        article(day) + "th"
    })
}


fn main() {

    for day in 1..13 {
        println!("On the {} day of Christmas, my true love gave to me:", ordinal(day));
        for gift_day in (1..(day+1)).rev() {
            println!("{} {}", article(gift_day), gift(gift_day))
        }
    }
}
