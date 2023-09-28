use rdev::{listen, Event};

pub fn callback(event: Event) {
    match event.name {
        Some(string) => println!("User wrote {:?}", string),
        None => (),
    }
}