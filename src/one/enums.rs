// Enums are a familiar concept from TypeScript...
// ... but don't underestimate them. In Rust they can
// carry data!

pub enum MyEnum {
    Plain,
    UnnamedValues(String, u64),
    NamedValues { foo: String, bar: u64 }, // basically a struct, but can be differentiated
}

impl MyEnum {
    // And they can have an `impl` too!
    fn say_hi(&self) {
        println!("hi!");
    }
}

pub enum Event {
    UserCreated {
        name: String,
        id: String,
        email: String,
    },
    UserDeleted(String),
    OrgCreated {
        name: String,
        id: String,
        address: String,
    },
    OrgDeleted(String),
}

pub struct MyDb;

impl Event {
    pub fn load(&self, _data: Vec<u8>) -> Event {
        // deserialize event
        Event::OrgDeleted("".into())
    }
    pub fn store(&self, _database: &MyDb) -> Result<(), String> {
        // write data to database
        //let blob = serialize_to_vec(self);
        //database.store(blob);
        Ok(())
    }
}
