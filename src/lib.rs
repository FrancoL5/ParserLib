use crate::logs::output_error;
use data_processor::CsvParserBuilder;
use files::FileBuilder;
use std::fs::{self, OpenOptions};
use std::io::prelude::*;

mod data_processor;
mod files;
mod logs;

pub fn execute(
    csv_file: Option<&str>,
    dir_path: Option<&str>,
    delete: bool,
    date_limit:bool
) -> Result<(), std::io::Error> {
    let config_file = FileBuilder::new(".").add_data("config.txt").build();
    let output = format!("{}/{}", config_file.data(), "result.txt");

    let dir_path = match dir_path {
        Some(dir) => dir,
        None => "archivos",
    };

    match csv_file {
        Some(csv) => {
            let file = FileBuilder::new(dir_path).add_data(csv).build();

            let mut data_lines = file.data().lines();

            let columns = &data_lines.next().unwrap().replace("\"", "");

            let parser = CsvParserBuilder::new()
                .set_columns(&columns)?
                .data(data_lines.collect())
                .build();

            let parsed = parser.parse(date_limit);
            match write_to(&*parsed, &output, false) {
                Ok(()) => (),
                Err(err) => output_error(
                    err,
                    &format!("Problema al escribir el archivo en:{}", output),
                ),
            };
            match write_to(&*create_backup(&*parsed), "./bk.txt", true) {
                Ok(()) => (),
                Err(err) => output_error(
                    err,
                    &format!("Problema al escribir el archivo en:{}", output),
                ),
            };

            if delete {
                file.delete_file().unwrap();
            }
            return Ok(());
        }
        None => {
            let files = match file_finder(dir_path) {
                Ok(value) => value,
                Err(err) => {
                    output_error(err, &format!("Direccion de la carpeta:{}", dir_path));
                    panic!("Falla en encontrar la carpeta");
                }
            };

            for file_name in files.iter() {
                let file = FileBuilder::new(dir_path).add_data(&file_name).build();

                let mut data_lines = file.data().lines();

                let columns = &data_lines.next().unwrap().replace("\"", "");

                let parser = CsvParserBuilder::new()
                    .set_columns(&columns)?
                    .data(data_lines.collect())
                    .build();

                let parsed = parser.parse(date_limit);

                match write_to(&*parsed, &output, false) {
                    Ok(()) => (),
                    Err(err) => output_error(
                        err,
                        &format!("Problema al escribir el archivo en:{}", output),
                    ),
                }
                match write_to(&*create_backup(&*parsed), "./bk.txt", true) {
                    Ok(()) => (),
                    Err(err) => output_error(
                        err,
                        &format!("Problema al escribir el archivo en:{}", output),
                    ),
                }

                if delete {
                    file.delete_file().unwrap();
                }
            }

            return Ok(());
        }
    }
}

pub fn file_finder(dir_path: &str) -> Result<Vec<String>, std::io::Error> {
    let entrys = fs::read_dir(dir_path)?;
    let mut result = vec![];
    for entry in entrys {
        let entry = entry?;
        let path = entry.file_name();

        let file_name = path.to_str().unwrap().to_owned();
        if file_name.contains(".csv") {
            result.push(file_name);
        }
    }
    Ok(result)
}
fn create_backup<'a>(data: impl Into<&'a str>) -> String {
    format!(
        "--Fecha de parseo {} --\n{}",
        chrono::Local::now(),
        data.into()
    )
}

fn write_to<'a>(data: impl Into<&'a str>, path: &str, append: bool) -> Result<(), std::io::Error> {
    let data: &str = data.into();
    let mut file = OpenOptions::new()
        .write(true)
        .append(append)
        .truncate(!append)
        .create(true)
        .open(path)
        .unwrap();
    writeln!(file, "{}", data)
}

#[cfg(test)]
mod tests {
    use crate::execute;

    #[test]
    fn parsear_limite() {
        execute(
            Some("Reportes_Checks (3) - copia.csv"),
            Some("C:/Users/franco luna/Downloads"),
            true,
            true
        )
        .unwrap()
    }
    #[test]
    fn parsear_sin_limite() {
        execute(
            Some("Reportes_Checks (4).csv"),
            Some("C:/Users/franco luna/Downloads"),
            true,
            false
        )
        .unwrap()
    }
}
