mod store;
use store::Store;

fn main() {
    let mut store = Store::new();

    let key1 = "key1".to_string();
    let val1 = "val1".to_string();

    let key2 = "key2".to_string();
    let val2 = "val2".to_string();

    println!("Inserting - {}-{}, {}-{}", key1, val1, key2, val2);
    store.set(&key1, &val1);
    store.set(&key2, &val2);

    println!("Getting key1");
    if let Some(got_val) = store.get(&key1) {
        println!("key: {}, val: {}", key1, got_val);
    } else {
        println!("key: {} not found", key1);
    }

    println!("Getting key2");
    if let Some(got_val) = store.get(&key2) {
        println!("key: {}, val: {}", key2, got_val);
    } else {
        println!("key: {} not found", key2);
    }

    println!("Deleting key2: {}", key2);
    store.del(&key2);
    println!("Getting key2 after deletion");
    if let Some(got_val) = store.get(&key2) {
        println!("key: {}, val: {}", key2, got_val);
    } else {
        println!("key: {} not found", key2);
    }
}
