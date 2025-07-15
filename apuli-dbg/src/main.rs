use apuli_lib::apuli::query;

fn main() {
    let result = query(&[], None, None, 5);
    let first = result.first().unwrap();
    println!("{first}");
}
