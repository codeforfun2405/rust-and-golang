use about_rust::{
    abs::{run_apps, AppRunner, IPhone, MacOS},
    collections, errors,
    functions::print_data,
    primitive::{data, strings, Status},
    structure::Person,
};

fn main() {
    println!("Hello, world!");

    data();
    strings();
    print_data("Nice to meet you");

    println!("person: {:?}", Person::new("Alex".to_string(), 20));

    println!("pending: {:?}", Status::Pending);

    let run_envs: Vec<Box<dyn AppRunner>> = vec![Box::new(MacOS {}), Box::new(IPhone {})];
    run_apps(run_envs);

    sum_data();

    println!("vec: {:?}", collections::vec());
    println!(
        "hash_map: {:?}",
        collections::hash_map(vec![("A".to_string(), 1), ("B".to_string(), 2)])
    );
}

fn sum_data() {
    match errors::sum_data(vec![1, 2, 3]) {
        Ok(num) => println!("success: {}", num),
        Err(e) => println!("failed: {}", e),
    }

    match errors::sum_data(vec![1, -1, 3]) {
        Ok(num) => println!("success: {}", num),
        Err(e) => println!("failed: {}", e),
    }
}
