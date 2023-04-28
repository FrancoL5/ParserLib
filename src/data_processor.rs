use std::ops::Deref;

pub struct CsvParserBuilder<'a> {
    columns: Vec<&'a str>,
    data: Vec<&'a str>,
}

impl<'a> CsvParserBuilder<'a> {
    pub fn new() -> Self {
        Self {
            columns: vec![],
            data: vec![],
        }
    }

    pub fn data(mut self, data: Vec<&'a str>) -> Self {
        self.data = data;
        self
    }

    pub fn set_columns(mut self, data: &'a str) -> Self {
        self.columns = data.split(",").collect();
        self
    }

    pub fn build(self) -> CsvParser<'a> {
        CsvParser {
            columns: self.columns,
            data: self.data,
        }
    }
}

pub struct CsvParser<'a> {
    pub columns: Vec<&'a str>,
    pub data: Vec<&'a str>,
}

impl<'a> CsvParser<'a> {
    pub fn parse(&self) -> String {
        let mut parsed: Vec<String> = Vec::with_capacity(self.data.capacity());

        'linea: for lineas in self.data.iter() {
            let temporary = lineas.replace("\"", "");
            let lineas_aux: Vec<&str> = temporary.split(",").collect();
            let mut aux = vec![];

            for (index, column) in self.columns.iter().enumerate() {
                match column.deref() {
                    "Número interno" => {
                        if lineas_aux[index].is_empty() {
                            continue 'linea;
                        }
                        aux.push(lineas_aux[index].to_owned())
                    }
                    "Fecha" => aux.push(lineas_aux[index].replace("-", "/").to_owned()),
                    "Horario" => aux.push(lineas_aux[index].to_owned()),
                    "Tipo" => {
                        let mut tipo = lineas_aux[index].to_owned();
                        if tipo == "In" {
                            tipo = tipo.replace("In", "E");
                        } else {
                            tipo = tipo.replace("Out", "S");
                        }
                        aux.push(tipo)
                    }
                    "Sucursal / HO" => {
                        let reloj = self.num_reloj(lineas_aux[index]);
                        if reloj == "not" {
                            continue 'linea;
                        }
                        aux.push(reloj);
                    }
                    _ => {}
                }
            }
            aux.swap(3, 4);
            parsed.push(aux.join(" "))
        }
        parsed.join("\n")
    }

    fn num_reloj(&self, area: impl Into<String>) -> String {
        let area = area.into();

        match area.as_str() {
            "Sucursal Córdoba" => "010",
            "Sucursal Tucumán" => "009",
            "Sucursal Salta" => "011",
            "Sucursal Rosario" => "012",
            "Sucursal Resistencia" => "008",
            "Sucursal Mendoza" => "002",
            "Sucursal Pcia de Bs As" => "004",
            "Casa Central" => "001",
            _ => "not",
        }
        .to_string()
    }
}
