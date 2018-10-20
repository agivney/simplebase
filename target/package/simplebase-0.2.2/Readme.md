simplebase
==========

A simple to use database library designed to have no dependencies

### Documentation

There is extensive documentation included with the source code and it can be rendered to html by running the 'cargo doc' from in the relevant directory of your project. It will be saved in the target/doc section of your project.

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
simplebase = "0.2.2"
```


The idea behind this crate is to have simple database capabilities for a project. It is also designed to be extremely easy to use and to have thorough documentation with lots of examples. An example session would be as follows:

```rust

extern crate simplebase;
use simplebase::engine::*;


fn main() {
let mut database = new_empty_database();

database.add_record_with_key("mob".to_string(), "0404111222".to_string());

database.add_record("This is a test".to_string());
database.add_record(0.23423 as f32);
database.add_record(0.23423 as f64);
database.add_record(23423 as u32);
database.add_record(23423 as u64);
database.add_record(-23423 as i32);
database.add_record(-23423 as i64);



database.add_record("Sam goes to the greatest market 1".to_string());
database.add_record("Sam goes to the greatest market 2".to_string());
database.add_record("Sam goes to the greatest market 3".to_string());
database.add_record("Sam goes to the greatest market 4".to_string());
database.add_record("Sam goes to the greatest market 5".to_string());
database.add_record_with_key("mob".to_string(), "0404111222".to_string());
database.add_record_with_key("test".to_string(), "Sam goes to the greatest market 5".to_string());
database.save_database("test5base.txt");


let loaded_database_read_only = load_hash_database_read_only("test5base.txt");
let _result = loaded_database_read_only.find("greatest");
let _result2 = loaded_database_read_only.get_record(4);
database.delete_record(4);
let _result3 = database.get_record(4);
database.save_database("test5base.txt");

}

```

In this release one can directly stores String,f32,f64,u32,u64,i32,i64. In the next release the aim is to be able to directly receive the same types back. Currently the data is returned as a String type.


### License

This project is licensed under either of

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)
