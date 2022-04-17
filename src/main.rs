use fileprocessor::file_process;
use fileprocessor::file_process::Action;
use fileprocessor::lib_properties;
use fileprocessor::lib_properties::Items;
use lib_properties::ProperyReader;
//mod lib;

fn main() {
    println!("Hello, world!");
    file_process::testfn();
    let fp = file_process::FileProcessor::new("testfile".to_string(), Action::Read);
    if fp.exist() {
        fp.read_file();
    } else {
        println!("file not found..");
    }
    let fp2 = file_process::FileProcessor::new("testfile".to_string(), Action::Write);
    fp2.write_file("test123");
    //mytest();

    let mut pr = ProperyReader::new("properties".to_string());
    let mut  ans =pr.get_properties();
    println!("{:?}", ans);
    let new1:Items = Items{ name:"haha".to_string(),value: "1".to_string() };
    ans.push(new1);
    pr.update_properties(ans);


}
