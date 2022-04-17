pub mod lib_properties;

use easy_reader::EasyReader;
use std::io::Write;
use std::path::Path;
use std::{
    fs::File,
    io::{self, Error},
};
pub mod file_process {
    use easy_reader::EasyReader;
    use std::fmt;
    use std::fs::File;
    use std::io::{Error, Write};
    use std::path::Path;

    pub fn testfn() {
        println!("test fn");
    }
    pub enum Action {
        Read,
        Write,
    }


    pub struct CustomError;

    impl fmt::Display for CustomError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid error")
        }
    }

    impl fmt::Debug for CustomError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid error")
        }
    }

    pub struct FileProcessor {
        name: String,
        action: Action,
    }

    impl FileProcessor {
        pub fn new(name: String, action: Action) -> FileProcessor {
            FileProcessor { name, action }
        }

        pub fn read_file(&self) -> Result<(), Error> {
            let file = File::open(&self.name.as_str())?;
            let mut reader = EasyReader::new(file)?;
            while let Some(line) = reader.next_line()? {
                println!("{}", line);
            }

            Ok(())
        }

        pub fn exist(&self) -> bool {
            let res = Path::new(&self.name).exists();
            return res;
        }

        pub fn write_file(&self, content: &str) -> Result<(), CustomError> {
            match &self.action {
                Action::Read => {
                    return Err(CustomError);
                }
                Action::Write => {}
            }

            let mut file = File::create(&self.name.as_str());
            match file {
                Ok(mut v) => {
                    v.write(content.as_ref());
                    Ok(())
                }
                Err(_) => {
                    return Err(CustomError);
                }
            }
        }


    }
}
