fn main() {
    // ------ Vector ------
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];    // will crash the program
    let does_not_exist = v.get(100); // will return None and continue execution

    // This example breaks compilation due to borrowing of mutable reference
    // let mut v = vec![1, 2, 3, 4, 5];
    //
    // let first = &v[0];
    //
    // v.push(6);
    //
    // println!("The first element is: {}", first);

    let v = vec![42, 420, 69];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![42, 420, 69];
    for i in &mut v {
        *i += 50;
    }
    dbg!(v);


    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // ------ String ------

    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial content");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    let hello = String::from("Hej");

    let mut s = String::from("foo");
    s.push_str("bar"); // `push_str` takes a &str

    let mut s = String::from("lo");
    s.push('l'); // `push` takes a single character

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // the `+` operator uses the `add` function whose signature looks something like:
                       // fn add(self, s: &str) -> String
                       // So the s1 has to be borrowed
                       // Effectively s3 gains ownership of s1 with s2 appended to it

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // This uses references for all of the variables.
                                             // No ownership is lost

    let s1 = String::from("hello");
    // let h = s1[0]; // This does not work in Rust

    let hello = "Здравствуйте";
    // let answer = &hello[0]; // Does not compile. Would return the first byte of a two byte character

    let hello = "Здравствуйте";
    let s = &hello[0..4]; // Will compile, but could crash at runtime if slicing in the middle
                          // of a UTF-8 character
    // let s = &hello[0..1]; // Like so
    // Be cautious when using ranges to create string slices

    for c in "日本語".chars() {
        println!("{}", c);
    }

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // ------ Hash Maps ------
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 10);
    dbg!(scores);

    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    dbg!(scores);

    let field_name = String::from("Favourite colour");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // score will be Some(&10). get return Option<&V>

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // overwriting a value

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 69);

    println!("{:?}", scores);

    // only inserting a value if the key has no value

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(69);

    println!("{:?}", scores);

    // updating a value based on the old value

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // or_insert returns &mut V
        *count += 1;                              // dereferencing necessary 
    }

    println!("{:?}", map);
}
