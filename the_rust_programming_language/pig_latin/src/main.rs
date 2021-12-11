fn main() {
    println!(
        "{}",
        convert_to_pig_latin("herp derp schmerp absolute unit")
    );
    println!(
        "{}",
        convert_to_pig_latin("Sten och sand och makadam här kommer Örjans helhäftiga barnprogram!")
    );
}

fn convert_to_pig_latin(s: &str) -> String {
    let vowels = "aeiouyåäöAEIOYÅÄÖ";

    let mut pig = String::new();

    for word in s.split_whitespace() {
        let c = word.chars().next().unwrap();
        if vowels.contains(c) {
            pig.push_str(word);
            pig.push_str("-hay");
        } else {
            let mut chars = word.chars();
            chars.next();

            pig.push_str(chars.as_str());
            pig.push_str("-");
            pig.push(c);
            pig.push_str("ay");
        }
        pig.push_str(" ");
    }

    pig
}
