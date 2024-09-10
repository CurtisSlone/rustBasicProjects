use singleton_pattern::get_logger;

fn main(){
    let logger = get_logger();

    {
        let mut logger = logger.lock().unwrap();
        logger.log("Hello from main thread");
    }

    {
        let mut logger = logger.lock().unwrap();
        logger.set_log_level("DEBUG".to_string());
        logger.log("This is debug level log");
    }

    {
        let mut logger = logger.lock().unwrap();
        logger.set_log_level("ERROR".to_string());
        logger.log("This is an error level log");
    }
}