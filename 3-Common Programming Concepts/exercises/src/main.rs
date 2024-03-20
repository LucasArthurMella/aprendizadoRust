fn main() {
    let farenheit = convertCelsiusToFarenheit(60.0);
    println!("{farenheit}");

    let celsius = convertFarenheitToCelsius(140.0);
    println!("{celsius}");

    let fibonnaci = getFibbonaci(6);
    println!("{fibonnaci}");

    getLyric();
}

fn convertCelsiusToFarenheit(celsius: f32) -> f32{
    let farenheit = (1.8 * celsius) + 32.0;
    farenheit
}


fn convertFarenheitToCelsius(farenheit: f32) -> f32{
    let celsius = (farenheit - 32.0) * (5.0/9.0);
    celsius
}

fn getFibbonaci(n: i32) -> i32{
    if n == 0 {
        return 0;
    }

    let mut fibonnaci = 0;
    let mut last_number = 1;

    for number in 1..n+1{
        let cur_fibbonaci = fibonnaci;
        fibonnaci = fibonnaci + last_number;
        last_number = cur_fibbonaci;
    }

    return fibonnaci;
}

fn getLyric(){
    let first = ("first", "A partridge in a pear tree. \n");
    let true_love = "my true love gave to me ";
    // let mut text = "On the " + first.0  + " day of Christmas, " + {true_love} + " ",{first.1};

    let mut text = String::new();
    text.push_str("On the ");
    text.push_str(first.0);
    text.push_str(" day of Christmas, \n");
    text.push_str(true_love);
    text.push_str("\n");
    text.push_str(first.1);
    text.push_str("\n");
    text.push_str("    \n");

    let lyrics_tuple_array = [
        ("second", "Two turtle doves, \n"),
        ("third", "Three French Hens, \n"),
        ("fourth", "Four calling birds, \n"),
        ("fifth", "Five golden rings, \n"),
        ("sixth", "Six geese a-laying, \n"),
        ("seventh", "Seven swans a-swimming, \n"),
        ("eighth", "Eight maids a-milking, \n"),
        ("ninth", "Nine ladies dancing, \n"),
        ("tenth", "Ten lords a-leaping, \n"),
        ("eleventh", "Eleven pipers piping, \n"),
        ("twelfth", "Twelve drummers drumming, \n")
    ];

    for (index, value) in lyrics_tuple_array.iter().enumerate() {
        text.push_str("On the ");
        text.push_str(value.0);
        text.push_str(" day of Christmas, \n");
        text.push_str(true_love);
        text.push_str("\n");

        for cur_index in (0..index+1).rev(){
            text.push_str(lyrics_tuple_array[cur_index].1);
        }

        text.push_str("And a partridge in a pear tree. \n");
        text.push_str("    \n");

    }

    println!("{text}");
}