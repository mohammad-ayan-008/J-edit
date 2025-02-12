use std::fs::read_to_string;
pub struct Buffer{
    pub file:Option<String>,
    pub lines:Vec<String>
}

impl Buffer {
    pub fn from_file(file:Option<String>)-> Self{
        let lines = match &file {
            None => vec![],
            Some(files) => read_to_string(files).unwrap().lines().map(|s| s.to_string()).collect()  ,
        };
        Self{
            file,
            lines
        }
    }

}