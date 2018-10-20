use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;

use file_services;
use std::convert::Into;

#[derive(Debug)]
pub struct RecordCharacteristics {
    pub record_id: usize,
    pub chksum: u32,
    pub datatype: u8,
    pub location: usize,
    pub size: usize,
    pub record: String,
    pub key: String,
}

#[derive(Debug)]
pub enum MyOutput {
    StringType(String),
    F64Type(f64),
    U64Type(u64),
    I64Type(i64),
    F32Type(f32),
    U32Type(u32),
    I32Type(i32),
}

impl Into<Option<f64>> for MyOutput {
    fn into(self) -> Option<f64> {
        match self {
            MyOutput::F64Type(u) => Some(u),
            _ => None,
        }
    }
}

// pub fn auto_converter(i: u8, value: String) -> MyOutput {
//     match i {
//         1 => MyOutput::StringType(value),
//         2 => MyOutput::F64Type(value.parse::<f64>().unwrap()),
//         _ => MyOutput::F64Type(0.56),
//     }
// }

// pub trait Conversion {

//     fn conversion(self) -> f64;

// }

// impl Conversion for String {
//     fn conversion(self)-> f64{

//         0.324234 as f64

//     }

// }

///This is enum helps categorise the type of data that is being stored. It will be used to convert the data
///back to its orignal type (this will be fully implimented in the next release).

pub enum DataType {
    Empty,
    StringType,
    F64Type,
    U64Type,
    I64Type,
    F32Type,
    U32Type,
    I32Type,
}

///This function converts the DataType enum to a numeric u8 value suitable for storage in the database.

pub fn data_type(datatype: DataType) -> u8 {
    match datatype {
        DataType::Empty => 0,
        DataType::StringType => 1,
        DataType::F64Type => 2,
        DataType::U64Type => 3,
        DataType::I64Type => 4,
        DataType::F32Type => 5,
        DataType::U32Type => 6,
        DataType::I32Type => 7,
    }
}

pub trait Base {
    fn addb(self) -> (DataType, String);
}

//#[derive(Clone, Copy)]
pub struct DataLink<T: Base> {
    pub what_to_add: T,
}
#[derive(Debug)]
pub struct RecordData {
    pub location: usize,
    pub location_type: u8,
    pub hash_data: HashMap<usize, RecordCharacteristics>,
    pub record_counter: usize,
    pub data_base: String,
}

pub struct RecordDataReadOnly {
    pub location: usize,
    pub location_type: u8,
    pub hash_data: HashMap<usize, RecordCharacteristics>,
    pub record_counter: usize,
    pub data_base: String,
}

/// Saves a database to a file.
///
/// # Examples
///
/// ```
/// use simplebase::engine::*;
/// let database = load_hash_database("test1base.txt");
/// database.save_database("test1base.txt");
///
/// ```

pub fn save_hash_database(filename: &str, hash_to_save: &HashMap<usize, RecordCharacteristics>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)
        .unwrap();
    let mut cache_write_hold = "".to_string();
    for individual_record_information in hash_to_save {
        cache_write_hold = cache_write_hold
            + &individual_record_information.0.to_string()
            + "~$"
            + &individual_record_information.1.record_id.to_string()
            + "~$"
            + &individual_record_information.1.chksum.to_string()
            + "~$"
            + &individual_record_information.1.datatype.to_string()
            + "~$"
            + &individual_record_information.1.location.to_string()
            + "~$"
            + &individual_record_information.1.size.to_string()
            + "~$"
            + &individual_record_information.1.record
            + "~$"
            + &individual_record_information.1.key
            + "~$";
    }
    file.write(cache_write_hold.as_bytes()).unwrap();
}

/// This function produces a basic chksum for a Vector of u8 bytes. It is not for security purposes but
/// rather data validation only.
///
/// # Examples
///
/// ```
/// let result = simplebase::engine::chksum(&"This is a test".as_bytes());
///
/// ```

pub fn chksum(data: &[u8]) -> u32 {
    let mut chksum = 0 as u32;
    for i in 0..data.len() {
        chksum += data[i] as u32;
    }
    return chksum;
}

/// This loads a file that was saved using the "save_database" as read only (only read functions such as searching etc are permitted)
/// function which can then  be used with the appropriate methods.
///
/// # Examples
///
/// ```
///
/// let loaded_hash_read_ony =  simplebase::engine::load_hash_database_read_only("test1base.txt");
///
/// ```
/// # Panics
/// If the file does not exist then this function will panic!()

pub fn load_hash_database_read_only(database_name: &str) -> RecordDataReadOnly {
    let mut loaded_hash: HashMap<usize, RecordCharacteristics> = HashMap::new();
    let raw_hash_file = file_services::open_file(&database_name).unwrap();
    let raw_hash_file = String::from_utf8_lossy(&raw_hash_file);
    let pre_translated_hash_data: Vec<&str> = raw_hash_file.split("~$").collect();

    // if pre_translated_hash_data.len() < 8 {
    //     return loaded_hash;
    // }

    let mut counter = 0;

    while counter < pre_translated_hash_data.len() - 1 {
        let mut database_hold = RecordCharacteristics {
            record_id: 0,
            chksum: 0,
            datatype: 0,
            location: 0,
            size: 0,
            record: "NULL".to_string(),
            key: "".to_string(),
        };

        database_hold.record_id = pre_translated_hash_data[counter + 1]
            .parse::<usize>()
            .unwrap();

        database_hold.chksum = pre_translated_hash_data[counter + 2]
            .parse::<u32>()
            .unwrap();
        database_hold.datatype = pre_translated_hash_data[counter + 3].parse::<u8>().unwrap();
        database_hold.location = pre_translated_hash_data[counter + 4]
            .parse::<usize>()
            .unwrap();

        database_hold.size = pre_translated_hash_data[counter + 5]
            .parse::<usize>()
            .unwrap();
        database_hold.record = pre_translated_hash_data[counter + 6].to_string();
        database_hold.key = pre_translated_hash_data[counter + 7].to_string();

        loaded_hash.insert(
            pre_translated_hash_data[counter].parse::<usize>().unwrap(),
            database_hold,
        );

        counter += 8;
    }
    //loaded_hash

    let mut record_counter2 = 0;

    match loaded_hash.get(&0) {
        Some(first_record) => record_counter2 = first_record.record_id, //this is a special value for the first record
        None => (),
    }

    RecordDataReadOnly {
        location: 0,
        location_type: 0,
        record_counter: record_counter2,
        hash_data: loaded_hash,
        data_base: "".to_string(),
    }
}

/// This loads a file that was saved using the "save_database"  function which can then  be used with the appropriate methods.
///
/// # Examples
///
/// ```
///
/// let loaded_hash =  simplebase::engine::load_hash_database("test1base.txt");
///
/// ```
/// # Panics
/// If the file does not exist then this function will panic!()

pub fn load_hash_database(database_name: &str) -> RecordData {
    let mut loaded_hash: HashMap<usize, RecordCharacteristics> = HashMap::new();
    let raw_hash_file = file_services::open_file(&database_name).unwrap();
    let raw_hash_file = String::from_utf8_lossy(&raw_hash_file);
    let pre_translated_hash_data: Vec<&str> = raw_hash_file.split("~$").collect();
    //println!("Debug 555 {:?}", pre_translated_hash_data);

    // if pre_translated_hash_data.len() < 8 {
    //     return loaded_hash;
    // }

    let mut counter = 0;

    while counter < pre_translated_hash_data.len() - 1 {
        let mut database_hold = RecordCharacteristics {
            record_id: 0,
            chksum: 0,
            datatype: 0,
            location: 0,
            size: 0,
            record: "NULL".to_string(),
            key: "".to_string(),
        };

        database_hold.record_id = pre_translated_hash_data[counter + 1]
            .parse::<usize>()
            .unwrap();

        database_hold.chksum = pre_translated_hash_data[counter + 2]
            .parse::<u32>()
            .unwrap();
        database_hold.datatype = pre_translated_hash_data[counter + 3].parse::<u8>().unwrap();
        database_hold.location = pre_translated_hash_data[counter + 4]
            .parse::<usize>()
            .unwrap();

        database_hold.size = pre_translated_hash_data[counter + 5]
            .parse::<usize>()
            .unwrap();
        database_hold.record = pre_translated_hash_data[counter + 6].to_string();
        database_hold.key = pre_translated_hash_data[counter + 7].to_string();

        loaded_hash.insert(
            pre_translated_hash_data[counter].parse::<usize>().unwrap(),
            database_hold,
        );

        counter += 8;
    }

    let mut record_counter2 = 0;

    match loaded_hash.get(&0) {
        Some(first_record) => record_counter2 = first_record.record_id, //this is a special value for the first record
        None => (),
    }

    RecordData {
        location: 0,
        location_type: 0,
        record_counter: record_counter2,
        hash_data: loaded_hash,
        data_base: "".to_string(),
    }
}

impl Base for i64 {
    fn addb(self) -> (DataType, String) {
        (DataType::I64Type, self.to_string())
    }
}

impl Base for String {
    fn addb(self) -> (DataType, String) {
        (DataType::StringType, self)
    }
}

impl Base for u64 {
    fn addb(self) -> (DataType, String) {
        (DataType::U64Type, self.to_string())
    }
}

impl Base for f64 {
    fn addb(self) -> (DataType, String) {
        (DataType::F64Type, self.to_string())
    }
}

impl Base for u32 {
    fn addb(self) -> (DataType, String) {
        (DataType::U32Type, self.to_string())
    }
}

impl Base for f32 {
    fn addb(self) -> (DataType, String) {
        (DataType::F32Type, self.to_string())
    }
}

impl Base for i32 {
    fn addb(self) -> (DataType, String) {
        (DataType::I32Type, self.to_string())
    }
}

impl Default for RecordDataReadOnly {
    fn default() -> RecordDataReadOnly {
        let mut record_counter = 0;
        let empty_hash = load_hash_database("empty_database.txt");
        let empty_hash = empty_hash.hash_data;

        match empty_hash.get(&0) {
            Some(first_record) => record_counter = first_record.record_id, //this is a special value for the first record
            None => (),
        }

        let database_contents = "".to_string();

        RecordDataReadOnly {
            location: 0,
            location_type: 0,
            record_counter: record_counter,
            hash_data: empty_hash,
            data_base: database_contents,
        }
    }
}

impl Default for RecordData {
    fn default() -> RecordData {
        let mut record_counter = 0;
        let empty_hash = load_hash_database("empty_database.txt");
        let empty_hash = empty_hash.hash_data;

        match empty_hash.get(&0) {
            Some(first_record) => record_counter = first_record.record_id, //this is a special value for the first record
            None => (),
        }

        let database_contents = "".to_string();

        RecordData {
            location: 0,
            location_type: 0,
            record_counter: record_counter,
            hash_data: empty_hash,
            data_base: database_contents,
        }
    }
}

impl RecordDataReadOnly {
    /// Searches the database for a particular term and returns the matching record in a String vector
    /// consisting of two values for each match: 1) The record id 2) The contents of the matching record.
    /// This method is available on read only databases (all methods work on writeable databases).
    /// # Examples
    ///
    /// ```
    /// use simplebase::engine::*;
    /// let loaded_database_read_only = load_hash_database_read_only("test1base.txt");
    /// let found_records = loaded_database_read_only.find("great");
    ///```
    pub fn find(&self, what_to_find: &str) -> Vec<String> {
        let mut search_results: Vec<String> = Vec::new();

        for i in &self.hash_data {
            match i.1.record.find(&what_to_find) {
                Some(_where_in_filename) => {
                    println!("Record {}", i.1.record);
                    search_results.push(i.0.to_string());
                    search_results.push(i.1.record.clone());
                }
                None => (),
            }
        }
        search_results
    }

    /// This method returns a record. It returns an empty string if the record does not exist.
    /// This is also available if a database is opened using the load_hash_database_read_only
    /// method.
    ///
    /// # Examples
    ///
    /// ```
    /// use simplebase::engine::*;
    /// let loaded_database_read_only = load_hash_database_read_only("test1base.txt");
    /// let particular_record = loaded_database_read_only.get_record(1);
    ///
    ///

    pub fn get_record(&self, record_number: usize) -> String {
        match self.hash_data.get(&record_number) {
            Some(record) => return record.record.to_owned(),
            None => return "".to_string(),
        }
    }

    /// Searches the database based on key and returns the matching record associated with the key.
    /// The returned results are collacted in a String vector consisting of two values for each match:
    /// 1)The record id 2) The contents of the matching record.
    /// This method is also available on read only databases (all methods work on writeable databases).
    /// # Examples
    ///
    /// ```
    /// use simplebase::engine::*;
    /// let loaded_database_read_only = load_hash_database_read_only("test1base.txt");
    /// let found_records = loaded_database_read_only.find_key("mob");
    ///```

    pub fn find_key(&self, what_to_find: &str) -> Vec<String> {
        let mut search_results: Vec<String> = Vec::new();
        //let records: Vec<&str> = self.data_base.split("<E><S>").collect();
        for i in &self.hash_data {
            match i.1.key.find(&what_to_find) {
                Some(_where_in_filename) => {
                    // println!("Record {}", i.1.record);
                    search_results.push(i.0.to_string());
                    search_results.push(i.1.record.clone());
                }
                None => (),
            }
        }
        search_results
    }

    ///This function returns the data type (e.g String, u64, f64 etc) of a stored value. This is based on the DataType enum.
    ///
    /// # Examples
    ///
    /// ```
    /// use simplebase::engine::*;
    /// let loaded_database_read_only = load_hash_database_read_only("test1base.txt");
    /// loaded_database_read_only.return_data_type(1);
    /// ```
    pub fn return_data_type(&self, record_number: usize) -> u8 {
        match self.hash_data.get(&record_number) {
            Some(record) => return record.datatype.to_owned(),
            None => return 0,
        }
    }

    /// Calculates a simple chksum on the contents of a record and compares it to the stored
    /// chksum. This will return false if there is a mismatch. If the record does not exist,
    /// it will still return true.
    ///
    /// # Examples
    ///
    /// ```
    /// use simplebase::engine::*;
    /// let mut database = new_empty_database();
    /// database.add_record_with_key_exclusive("mob".to_string(), "0404111222".to_string());
    /// let result = database.verify_record(1);
    /// assert_eq!(true,result);
    ///
    /// ```

    pub fn verify_record(&self, record_number: usize) -> bool {
        match self.hash_data.get(&record_number) {
            Some(record) => {
                if chksum(&record.record.as_bytes()) == record.chksum {
                    return true;
                } else {
                    return false;
                }
            }
            None => return true,
        }
    }
}

impl RecordData {
    ///This method adds information to the database with a key as well.  
    ///
    /// # Examples
    ///
    /// ```
    /// use simplebase::engine::*;
    /// let mut loaded_database = load_hash_database("test1base.txt");
    /// loaded_database.add_record_with_key("mob".to_string(), "0404111222".to_string());
    ///
    /// ```
    pub fn add_record_with_key<T>(&mut self, key_to_add: String, record: T)
    where
        T: Base,
    {
        let (data_type_hold, what_to_save) = record.addb();

        self.record_counter += 1;

        let database_hold = RecordCharacteristics {
            record_id: self.record_counter,
            chksum: chksum(&what_to_save.as_bytes()),
            datatype: data_type(data_type_hold),
            location: 1,
            size: 0,
            record: what_to_save,
            key: key_to_add,
        };

        let counter_record = RecordCharacteristics {
            record_id: self.record_counter,
            chksum: 0,
            datatype: 0,
            location: 0,
            size: 0,
            record: "".to_string(),
            key: "".to_string(),
        };

        self.hash_data.insert(0, counter_record);

        self.hash_data.insert(self.record_counter, database_hold);
    }

    /// This method adds information to the database with a key as well. If the key already exists
    /// in the database, the information will not be added. The method returns true if successful,
    /// false, if they key exists already in the database.   
    ///
    /// # Examples
    ///
    /// ```
    /// use simplebase::engine::*;
    /// let mut loaded_database = load_hash_database("test1base.txt");
    /// loaded_database.add_record_with_key_exclusive("mob".to_string(), "0404111222".to_string());
    ///
    /// ```

    pub fn add_record_with_key_exclusive<T>(&mut self, key_to_add: String, record: T) -> bool
    where
        T: Base,
    {
        let (data_type_hold, what_to_save) = record.addb();

        self.record_counter += 1;

        let database_hold = RecordCharacteristics {
            record_id: self.record_counter,
            chksum: chksum(&what_to_save.as_bytes()),
            datatype: data_type(data_type_hold),
            location: 1,
            size: 0,
            record: what_to_save,
            key: key_to_add.clone(),
        };

        let counter_record = RecordCharacteristics {
            record_id: self.record_counter,
            chksum: 0,
            datatype: 0,
            location: 0,
            size: 0,
            record: "".to_string(),
            key: "".to_string(),
        };

        //This checks to see if the key already exists, if it does it will not add the record
        //to the database
        if self.find_key(&key_to_add).len() == 0 {
            //This line updates the record counter in the database
            self.hash_data.insert(0, counter_record);

            self.hash_data.insert(self.record_counter, database_hold);
            return true;
        } else {
            println!(
                "The key {} already exist, not adding the record to the database",
                key_to_add
            );
            return false;
        }
    }

    ///This method adds information to the database.  
    ///
    /// # Examples
    ///
    /// ```
    /// use simplebase::engine::*;
    /// let mut loaded_database = load_hash_database("test1base.txt");
    /// loaded_database.add_record("0404111222".to_string());
    ///
    /// ```
    pub fn add_record<T>(&mut self, record: T)
    where
        T: Base,
    {
        let (data_type_hold, what_to_save) = record.addb();

        self.record_counter += 1;

        let database_hold = RecordCharacteristics {
            record_id: self.record_counter,
            chksum: chksum(&what_to_save.as_bytes()),
            datatype: data_type(data_type_hold),
            location: 1,
            size: 0,
            record: what_to_save,
            key: "".to_string(),
        };

        let counter_record = RecordCharacteristics {
            record_id: self.record_counter,
            chksum: 0,
            datatype: 0,
            location: 0,
            size: 0,
            record: "".to_string(),
            key: "".to_string(),
        };

        self.hash_data.insert(0, counter_record);

        self.hash_data.insert(self.record_counter, database_hold);
    }

    /// This method deletes a record based on the supplied record number. If the record
    /// does not exist, it does nothing.
    ///
    /// # Examples
    ///
    /// ```
    /// use simplebase::engine::*;
    /// let mut database = load_hash_database("test1base.txt");
    /// database.delete_record(1);
    /// database.delete_record(1000000); //This will do nothing since the 1000000 record does not exist.
    /// ```

    pub fn delete_record(&mut self, record_number: usize) {
        self.hash_data.remove(&record_number);
    }

    ///    Searches the database for a particular term and returns the matching record in a String vector consisting of two values for each match 1) The record id 2) The contents of the matching record.
    ///     This method is available on read only databases (all methods work on writeable databases).
    /// # Examples
    ///
    /// ```
    /// use simplebase::engine::*;
    /// let loaded_database = load_hash_database("test1base.txt");
    /// let found_records = loaded_database.find("great");
    ///```

    pub fn find(&self, what_to_find: &str) -> Vec<String> {
        let mut search_results: Vec<String> = Vec::new();

        for i in &self.hash_data {
            match i.1.record.find(&what_to_find) {
                Some(_where_in_filename) => {
                    //println!("Record {}", i.0);
                    search_results.push(i.0.to_string());
                    search_results.push(i.1.record.clone());
                }
                None => (),
            }
        }
        search_results
    }

    ///  Searches the database key values (if a key value exists for a record- key values are optional) for a particular term and returns the
    ///matching record in a String vector consisting of two values for each match 1) The record id 2) The contents of the matching record.

    ///This method is available on read only databases (all methods work on writeable databases).
    /// # Examples
    ///
    /// ```
    /// use simplebase::engine::*;
    /// let loaded_database = load_hash_database("test1base.txt");
    /// let found_records = loaded_database.find_key("great");
    ///```

    pub fn find_key(&self, what_to_find: &str) -> Vec<String> {
        let mut search_results: Vec<String> = Vec::new();

        for i in &self.hash_data {
            match i.1.key.find(&what_to_find) {
                Some(_where_in_filename) => {
                    //println!("Record {}", i.1.record);
                    search_results.push(i.0.to_string());
                    search_results.push(i.1.record.clone());
                }
                None => (),
            }
        }
        search_results
    }

    /// This method returns a record. It returns an empty string if the record does not exist.
    /// This is also available if a database is opened using the load_hash_database_read_only
    /// method.
    ///
    /// # Examples
    ///
    /// ```
    /// use simplebase::engine::*;
    /// let loaded_database = load_hash_database("test1base.txt");
    /// let particular_record = loaded_database.get_record(1);
    ///
    ///

    pub fn get_record(&self, record_number: usize) -> String {
        match self.hash_data.get(&record_number) {
            Some(record) => return record.record.to_owned(),
            None => return "".to_string(),
        }
    }

    ///This function returns the data type (e.g String, u64, f64 etc) of a stored value. This is based on the DataType enum.
    ///
    /// # Examples
    ///
    /// ```
    /// use simplebase::engine::*;
    /// let mut database = load_hash_database("test1base.txt");
    /// database.return_data_type(1);
    /// ```
    pub fn return_data_type(&self, record_number: usize) -> u8 {
        match self.hash_data.get(&record_number) {
            Some(record) => return record.datatype.to_owned(),
            None => return 0,
        }
    }

    /// Saves a database hash table to a file. This will need to be run to save
    /// the database to a file. It is up to the user to impliment this action.
    ///
    /// # Examples
    ///
    /// ```
    /// use simplebase::engine::*;
    /// let database = load_hash_database("test1base.txt");
    /// database.save_database("test1base.txt");
    ///
    /// ```

    pub fn save_database(&self, filename: &str) {
        save_hash_database(filename, &self.hash_data);
    }

    /// Calculates a simple chksum on the contents of a record and compares it to the stored
    /// chksum. This will return false if there is a mismatch. If the record does not exist,
    /// it will still return true.
    ///
    /// # Examples
    ///
    /// ```
    /// use simplebase::engine::*;
    /// let mut database = new_empty_database();
    /// database.add_record_with_key_exclusive("mob".to_string(), "0404111222".to_string());
    /// let result = database.verify_record(1);
    /// assert_eq!(true,result);
    ///
    /// ```

    pub fn verify_record(&self, record_number: usize) -> bool {
        match self.hash_data.get(&record_number) {
            Some(record) => {
                if chksum(&record.record.as_bytes()) == record.chksum {
                    return true;
                //return ChksumResult::Pass;
                } else {
                    return false;

                    //return ChksumResult::Fail;
                }
            }
            None => return true, //return ChksumResult::Empty,
        }
    }
}

///This function creates a new empty database which is writable and readable.
///
/// # Examples
///
/// ```
/// use simplebase::engine::*;
/// let mut database = new_empty_database();
/// database.add_record(20);
/// ```

pub fn new_empty_database() -> RecordData {
    let data_type_location: HashMap<usize, RecordCharacteristics> = HashMap::new();

    RecordData {
        location: 0,
        location_type: 0,
        record_counter: 0,
        hash_data: data_type_location,
        data_base: "".to_string(),
    }
}
