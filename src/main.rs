use std::error::Error;


mod txt_writer;
mod csv_reader;
mod txt_reader;
mod event_watcher;

fn main() -> Result<(), Box<dyn Error>> {
    let path: String = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    
    let mut i = 0;

    println!("Watching {}", path);

    while i < 3 {
        let event_path = event_watcher::watch_folder_trigger(&path)?;

        let output = read_file(&event_path)?;
        println!("{:?}", output);

        // TODO: Remove hardcode of file name
        _ = convert_file("templates/cars.txt", output);

        i += 1;
        println!("{}", i);
        println!("Still watching: {}", &path);
    }

    Ok(())
}

fn read_file(file: &str) -> Result<(Vec<(String, String)>, usize), Box<dyn Error>> {
    let mut contents: (Vec<(String, String)>, usize) = (Vec::new(), 0);
    if file.contains(".csv") {
        contents = csv_reader::read_file_from_path(file)?;
        println!("{:?}", contents);
    }
    else if file.contains(".xlsx") {
        todo!();
    }
    else {
        panic!("Invalid file!")
    }

    Ok(contents)
}

fn convert_file(path: &str, values: (Vec<(String, String)>, usize)) -> Result<(), Box<dyn Error>>{
    let fields_to_replace = txt_reader::read_file_from_path(path)?;
    let mut converted = fields_to_replace.clone();
    let mut converted_end = String::new();
    let mut i = 0;

    for val in values.0 {
        let deliminated_field: String = format!("<{}>", val.0);
        
        converted = converted.to_string().replace(&deliminated_field, &val.1);

        if values.1-1 == i {
            println!("All converted: {}", converted);
            converted_end.push_str(&converted);
            i = 0;
            converted = fields_to_replace.clone();
        } 
        else {
            i += 1;
        }
    }

    _ = txt_writer::write_to_file("./output/".to_string(), converted_end.to_string());

    Ok(())
}
