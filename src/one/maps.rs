use std::collections::{BTreeMap, HashMap};

pub fn hurray_for_maps() {
    let map1 = BTreeMap::from([(1, "Hello"), (2, "World")]);
    let mut map2 = HashMap::new();

    // This is a good opportunity to talk about Type Inference
    // NOTE : Temporarily disable inlay hints to better see the distinction here
    //        mod + p - search for inlay, then disable
    let map3 = BTreeMap::<i32, &str>::from([(1, "Hello"), (2, "World")]);
    let map4 = BTreeMap::<i32, _>::from([(1, "Hello"), (2, "World")]);

    let map5 = BTreeMap::from([(1i32, "Hello"), (2i32, "World")]);

    map2.extend(map1.iter());

    map2.insert(3, "!");
    map2.retain(|_key, value| value.len() > 1);

    for (key, value) in map2.iter() {
        println!("{}: {}", key, value);
    }

    println!("Entry 4: {}", map2.entry(4).or_insert("!"));
}
