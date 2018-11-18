#[cfg(test)]

mod test {

    use engine::*;

    #[test]
    fn test_auto_converter() {
        let result = MyOutput::F64Type(0.45);

        println!("Auto Converter {:#?}", result);

        //assert_eq!(0.9087345978 as f64,result);
        // let mut database = new_empty_database();
        // database.add_record(20);
        // println!("Test Return datatype {}", database.return_data_type(1));
        // assert_eq!(database.get_record(1), "20".to_string());
    }

    #[test]
    fn test_new_empty_database() {
        let mut database = new_empty_database();
        database.add_record(20);
        println!("Test Return datatype {}", database.return_data_type(1));
        assert_eq!(database.get_record(1), "20".to_string());
    }

    #[test]
    fn test_add_record() {
        let mut database = new_empty_database();

        database.add_record("Sam goes to the  greatest market 3".to_string());
        database.add_record("dsafkljh asdhj asdflksjdalk sdfalkj".to_string());
        let result = database.find("greatest");
        assert_eq!(result[1], "Sam goes to the  greatest market 3".to_string());
        database.add_record("dsafkljh asdhj asdflksjdalk sdfalkj".to_string());
        let result = database.get_record(2);
        assert_eq!(result, "dsafkljh asdhj asdflksjdalk sdfalkj".to_string());
    }

    #[test]
    fn test_delete_record() {
        let mut database = new_empty_database();

        database.add_record("Sam goes to the  greatest market 3".to_string());
        database.add_record("dsafkljh asdhj asdflksjdalk sdfalkj".to_string());
        let result = database.find("greatest");
        assert_eq!(result[1], "Sam goes to the  greatest market 3".to_string());
        database.add_record("dsafkljh asdhj asdflksjdalk sdfalkj".to_string());
        let result = database.get_record(2);
        assert_eq!(result, "dsafkljh asdhj asdflksjdalk sdfalkj".to_string());
        println!("Pre delete record {:#?}", database);
        database.delete_record(1);
        println!("Pos delete record {:#?}", database);
        let result = database.find("greatest");
        assert_eq!(result.len(), 0);
    }
    #[test]
    fn test_save_and_load_database() {
        let mut database = new_empty_database();
        database.add_record_with_key("mob".to_string(), "0404111222".to_string());
        database.add_record("Sam goes to the  greatest market 1".to_string());
        database.add_record("Sam goes to the  greatest market 2".to_string());
        database.add_record("Sam goes to the  greatest market 3".to_string());
        database.add_record("Sam goes to the  greatest market 4".to_string());
        database.add_record("Sam goes to the  greatest market 5".to_string());
        database.add_record_with_key("mob".to_string(), "0404111222".to_string());
        database.add_record_with_key(
            "test".to_string(),
            "Sam goes to the  greatest market 5".to_string(),
        );
        database.save_database("test1base.txt");

        let loaded_database_read_only = load_hash_database_read_only("test1base.txt");
        let result = loaded_database_read_only.find("5");

        assert_eq!("Sam goes to the  greatest market 5".to_string(), result[1]);
    }

    #[test]
    fn test_find() {
        let mut database = new_empty_database();
        database.add_record("Sam goes to the  greatest market 3".to_string());
        let result = database.find("greatest");
        assert_eq!("Sam goes to the  greatest market 3".to_string(), result[1]);
    }

    #[test]
    fn test_find_key() {
        let mut database = new_empty_database();
        database.add_record_with_key("mob".to_string(), "0404111222".to_string());
        database.find_key("mob");
        let result = database.find_key("mob");
        assert_eq!("0404111222".to_string(), result[1]);
    }

    #[test]
    fn test_add_record_with_key() {
        let mut database = new_empty_database();

        database.add_record_with_key("mob".to_string(), "0404111222".to_string());
    }

    #[test]
    fn test_add_record_with_key_exclusive() {
        let mut database = new_empty_database();
        assert_eq!(
            true,
            database.add_record_with_key_exclusive("mob".to_string(), "0404111222".to_string())
        );
        assert_eq!(
            false,
            database.add_record_with_key_exclusive("mob".to_string(), "0404111222".to_string())
        );
    }

    #[test]
    fn test_typical_database_session() {
        let mut database = new_empty_database();

        database.add_record_with_key("mob".to_string(), "0404111222".to_string());
        database.add_record("Sam goes to the  greatest market 1".to_string());
        database.add_record("Sam goes to the  greatest market 2".to_string());
        database.add_record("Sam goes to the  greatest market 3".to_string());
        database.add_record("Sam goes to the  greatest market 4".to_string());
        database.add_record("Sam goes to the  greatest market 5".to_string());
        database.add_record_with_key("mob".to_string(), "0404111222".to_string());
        database.add_record_with_key(
            "test".to_string(),
            "Sam goes to the  greatest market 5".to_string(),
        );

        database.add_record(0.23423 as f32);
        database.add_record(0.23423 as f64);
        database.add_record(23423 as u32);
        database.add_record(23423 as u64);
        database.add_record(-23423 as i32);
        database.add_record(-23423 as i64);

        database.save_database("test2base.txt");

        let loaded_database_read_only = load_hash_database_read_only("test2base.txt");
        loaded_database_read_only.find("greatest");
        loaded_database_read_only.get_record(4);
        loaded_database_read_only.get_record(4);
        database.delete_record(4);

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
        database.add_record_with_key(
            "test".to_string(),
            "Sam goes to the greatest market 5".to_string(),
        );
        database.save_database("test5base.txt");

        let loaded_database_read_only = load_hash_database_read_only("test5base.txt");
        let _result = loaded_database_read_only.find("greatest");
        let _result2 = loaded_database_read_only.get_record(4);
        database.delete_record(4);
        let _result3 = database.get_record(4);
        database.save_database("test5base.txt");
    }

    #[test]
    fn test_verify_database() {
        let loaded_database = load_hash_database("test2base.txt");
        loaded_database.verify_database();
        //assert_eq!(true, result);
    }

    #[test]
    fn test_verify_record() {
        let mut database = new_empty_database();

        database.add_record_with_key_exclusive("mob".to_string(), "0404111222".to_string());
        let result = database.verify_record(1);
        assert_eq!(true, result);
    }

    #[test]
    fn test_check_types() {
        let mut database = new_empty_database();

        database.add_record("This is a test".to_string());
        database.add_record(0.23423 as f32);
        database.add_record(0.23423 as f64);
        database.add_record(23423 as u32);
        database.add_record(23423 as u64);
        database.add_record(-23423 as i32);
        database.add_record(-23423 as i64);

        assert_eq!(database.return_data_type(0), 0);
        assert_eq!(database.return_data_type(1), 1);
        assert_eq!(database.return_data_type(2), 5);
        assert_eq!(database.return_data_type(3), 2);
        assert_eq!(database.return_data_type(4), 6);
        assert_eq!(database.return_data_type(5), 3);
        assert_eq!(database.return_data_type(6), 7);
        assert_eq!(database.return_data_type(7), 4);
    }

}
