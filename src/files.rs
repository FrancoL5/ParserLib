use std::fs::{ self};


use crate::logs::output_error;
pub struct FileBuilder<'a> {
    file_dir: &'a str,
    file_name: Option<&'a str>,
    data: Option<String>,
}

impl<'a> FileBuilder<'a> {
    pub fn new(file_dir: &'a str) -> Self {
        Self {
            file_name: None,
            file_dir,
            data: None,
        }
    }

    pub fn add_data(mut self, file_name: &'a str) -> FileBuilder {
        let complete_path = format!("{}/{}", self.file_dir, file_name);
        let data = read_data(&complete_path);
        self.data = Some(data);
        self.file_name = Some(file_name);
        self
    }

    pub fn build(self) -> File {
        File {
            data: self.data.unwrap_or_else(|| "".into()),
            full_dir: format!("{}\\{}",self.file_dir,self.file_name.unwrap_or(""))
        }
    }
}

fn read_data(complete_path: &str) -> String {
    match fs::read_to_string(complete_path) {
        Err(msg) => {
            let arguments = format!("Error al leer el archivo en: {complete_path}");
            output_error(msg, &arguments);
            arguments
        }
        Ok(data) => data,
    }
}

pub struct File {
    data: String,
    full_dir: String
}

impl File {
    pub fn data(&self) -> &str {
        &self.data
    }

    pub fn delete_file(self) -> Result<(), std::io::Error> {
        fs::remove_file(self.full_dir)
    }

}
