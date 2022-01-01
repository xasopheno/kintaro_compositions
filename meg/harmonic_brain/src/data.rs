use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvData {
    pub data: Vec<f32>,
}

impl CsvData {
    pub fn get_data(filename: String) -> Result<Vec<Self>, Error> {
        let path = Path::new(&filename);
        let cwd = std::env::current_dir()?;
        let file = File::open(path).expect(
            format!(
                "unable to read file: {}. current working directory is: {}",
                path.display().to_string(),
                cwd.display().to_string(),
            )
            .as_str(),
        );
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b',')
            .from_reader(file);

        Ok(rdr
            .deserialize::<Self>()
            .map(|datum| datum.expect("Error deserializing datum"))
            .collect())
    }
}
