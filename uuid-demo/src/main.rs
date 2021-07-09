use uuid::adapter::Simple;
use uuid::Uuid;

fn main() {
    // version 4 uuid
    let uuid = new_version4_uuid();
    println!("{}", uuid);

    // simple version 4 uuid
    let simple_uuid = new_simple_version4_uuid();
    println!("{}", simple_uuid);
}

// 90661afa-715d-434f-af77-44dfc355bad6
pub fn new_version4_uuid() -> String {
    let uuid = Uuid::new_v4();
    uuid.to_string()
}

// 90661afa715d434faf7744dfc355bad6
pub fn new_simple_version4_uuid() -> String {
    let uuid = Uuid::new_v4();
    Simple::from_uuid(uuid).to_string()
}
