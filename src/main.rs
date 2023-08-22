use std::error::Error;
use dotenv::dotenv;

mod txt_writer;
mod csv_reader;
mod txt_reader;
mod event_watcher;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let template_loc = std::env::var("TEMPLATE_LOC").expect("TEMPLATE_LOC must be set.");
    let template_type = std::env::var("TEMPLATE_TYPE").expect("TEMPLATE_TYPE must be set.");
    let output_location = std::env::var("OUTPUT_LOC").expect("OUTPUT_LOC must be set.");
    let output_type = std::env::var("OUTPUT_TYPE").expect("OUTPUT_TYPE must be set.");

    println!("{}", template_loc);
    println!("{}", output_location);
    println!("{}", output_type);

    let path: String = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    
    let mut i = 0;

    println!("Watching {}", path);

    loop {
        let event_path = event_watcher::watch_folder_trigger(&path)?;

        let output = read_file(&event_path)?;
        println!("{:?}", output);

        // TODO: Remove hardcode of file name
        let file_name = get_file_name(&event_path)?;

        let template_path = format!("{}{}{}", template_loc, file_name, template_type);
        let output_path = format!("{}{}{}", output_location, file_name, output_type);

        _ = convert_file(&template_path, &output_path, output);

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

fn convert_file(template_path: &str, output_path: &str, values: (Vec<(String, String)>, usize)) -> Result<(), Box<dyn Error>>{
    dotenv().ok();

    let fields_to_replace = txt_reader::read_file_from_path(template_path)?;
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
    println!("{}", output_path);
    _ = txt_writer::write_to_file(output_path.to_string(), converted_end.to_string());

    Ok(())
}

fn get_file_name(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file: String = String::new();
    let mut file_name: String = String::new();
    let split_path = path.split("/");

    for spl in split_path {
        file = spl.to_string();
    }

    let split_file: Vec<&str>= file.split(".").collect();

    file_name = split_file[0].to_string();

    Ok(file_name)
}