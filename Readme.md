simplebase
==========

A simple to use database library

[![Build Status](https://travis-ci.com/agivney/simplebase.svg?branch=master)]

### Documentation

There is extensive documentation included with the source code and it can be rendered to html by running the 'cargo doc' from in the relevant directory of your project. It will be saved in the target/doc section of your project.

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
simplebase = "0.3.35"
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

Ver 0.3.35

-Removed dead_code message due to the obfuscate data function (which can either be used or ignored).
-Fixed spelling error inside readme.md.
-Did a simple review of the code one year on and ran cargo-clippy on the code.


Ver 0.3.34

-Added the save_database_every() function as an option so that it will only save the database every # cycles. This 
can reduce writes significantly in highly demanding situations. Please read the docs for the caviats.
-I added a file 5 milisecond sleep when a file lock is encounted, this reduces locking and frees up things so that 
a save can occurr. I did this since the file hammer testing was occasionally failing, since the write lock loop could
not breath and free things up. This measure appears to who have resolved this issue. I would strong advise upgrading to
this version.

Ver 0.3.33

-Reverted back to the original length function as records were being skipped.

Ver 0.3.32

-Fixed an inacuracy in the length() function which could cause in inacurate result after a live delete.
-Added some more functions to the test suite.

Pre 0.3.32

This latest release adds some minor breaking changes but I considered them important. The first is that file locking has been introduced for reading and saving, this means the fs2 crate is now used. I also added a hammer test, that hammers the database with 50 saves running on 2 threads simultanously (I tested it with 500 saves on my own computer on 2 simultaneous tests) to make sure the file locking is effective and that no database corruption was occuring under threaded conditions. It passes. 


The second is that if a database.get_record(23423) is made, and the record does not exist, a "None" is now returned instead of an empty string. I also added a few more functions to the test suite.










### License

This project is licensed under either of

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

