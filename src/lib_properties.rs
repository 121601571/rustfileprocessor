use easy_reader::EasyReader;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{Error, Write};
#[derive(Debug)]
pub struct Items {
    pub name: String,
    pub value: String,
}
// pub trait PropertyOperator {
//
//     fn read_cfg(&self) ->Vec<Items>;
//
//     fn write_cfg(&self) ->Vec<Items>;
//
// }

pub struct ProperyReader {
    name: String,
    status: i8,
    info: Vec<Items>,
}

impl ProperyReader {
    pub fn new(name: String) -> ProperyReader {
        ProperyReader {
            name,
            status: 1,
            info: vec![],
        }
    }

    pub fn get_properties(&mut self) -> Vec<Items> {
        &self.refresh();
        let res = self.make_copy();
        res
    }

    fn make_copy(&self) -> Vec<Items> {
        let mut ans = vec![];
        for i in &self.info {
            let it = Items {
                name: i.name.to_string(),

                value: i.value.to_string(),
            };
            ans.push(it);
        }
        ans
    }

    fn validate(&self, item: &Vec<Items>) -> bool {
        let mut hashset1 = HashSet::new();
        for i in item {
            let name1 = (*i.name).to_string();
            if hashset1.contains(&name1) {
                return false;
            }
            hashset1.insert(name1);
        }

        true
    }

    pub fn update_properties(&mut self, items: Vec<Items>) {
        let res = self.validate(&items);
        if res == false {
            return;
        }

        &self.info.clear();
        for i in items {
            &self.info.push(i);
        }
        //&self.info = items;
        self.write_file();
    }

    fn refresh(&mut self) {
        &self.read_file();
    }

    fn read_file(&mut self) -> Result<(), Error> {
        let file = File::open(&self.name.as_str())?;
        let mut reader = EasyReader::new(file)?;
        &self.info.clear();
        while let Some(line) = reader.next_line()? {
            if line.len() == 0 {
                continue;
            }
            let line1: Vec<&str> = line.split('=').collect();
            let it = Items {
                name: line1[0].to_string(),

                value: line1[1].to_string(),
            };
            &self.info.push(it);
        }
        Ok(())
    }

    fn write_file(&mut self) -> Result<(), Error> {
        let mut file = File::create(&self.name.as_str())?;
        for i in &self.info {
            let mut name = &i.name;
            let value = &i.value;
            let mut str1 = "".to_string();
            str1.push_str(name);
            str1.push('=');
            str1.push_str(name);
            str1.push_str("\n");
            file.write(str1.as_ref());
        }
        Ok(())
    }
}
