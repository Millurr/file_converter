use std::error::Error;

pub fn read_file_from_path(path: &str) -> Result<(Vec<(String, String)>, usize), Box<dyn Error>> {
    let mut res: Vec<(String, String)> = Vec::new();
    let mut reader = csv::Reader::from_path(path)?;

    let headers = reader.headers()?;
    let hdrs = headers.clone();

    for result in reader.records() {
        let record = result?;

        for i in 0..hdrs.len() {
            // println!("{:?}: {:?}", &hdrs[i].to_string(), &record[i].to_string());
            res.push((hdrs[i].to_string(), record[i].to_string()))
        }
    }

    Ok((res, hdrs.len()))
}