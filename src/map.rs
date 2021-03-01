use std::collections::HashMap;

fn maptest() {
    let mut map = HashMap::new();
    let mut go = vec!["Shindou Hikaru","Toya Akira"];
    let onepiece = vec!["Monkey D Luffy","Sanji"];
    go.push("Toya Koyo");
    let gokey = String::from("hikarunogo");
    let opkey = String::from("onepiece");
    map.insert(&gokey,go);
    map.insert(&opkey,onepiece);
    let x = map.entry(&gokey).or_insert(vec![]);
    *x = vec!["update go"];
    println!("{:?}", map);
}