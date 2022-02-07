use std::path::PathBuf;
use std::io;
use std::fs;
use std::time::*;
use std::str::FromStr;

use super::error::*;

#[derive(Debug)]
pub enum SizeOption {
    Small,
    Medium,
    Large,
}

impl FromStr for SizeOption {
    type Err = ImagixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "small" => Ok(SizeOption::Small),
            "medium" => Ok(SizeOption::Medium),
            "large" => Ok(SizeOption::Large),
            _ => Err(ImagixError::SizeOptionError("Invalied Size".to_string()))
        }
    }
}

#[derive(Debug)]
pub enum Mode {
    Single,
    All,
}

impl FromStr for Mode {
    type Err = ImagixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single" => Ok(Mode::Single),
            "all" => Ok(Mode::All),
            _ => Err(ImagixError::ModeOptionError("Invalied Size".to_string()))
        }
    }
}

pub fn process_resize_request(size: SizeOption, mode: Mode, src_directory: &mut PathBuf)
    -> Result<(), ImagixError> {
    
    let size = match size {
        SizeOption::Small => 200,
        SizeOption::Medium => 400,
        SizeOption::Large => 800,
    };

    match mode {
        Mode::Single => {
            resize_single(size, src_directory)
        },
        Mode::All => {
            resize_all(size, src_directory)
        },
    }
}

pub fn resize_single(size: u32, src_directory: &mut PathBuf) -> Result<(), ImagixError> {
    resize_image(size, src_directory)
}

pub fn resize_all(size: u32, src_directory: &mut PathBuf) -> Result<(), ImagixError> {
    let files = get_image_files(src_directory.to_path_buf()).unwrap();

    for mut file in files {
        resize_image(size, &mut file).unwrap();
    }

    Ok(())
}

pub fn get_image_files(src_folder: PathBuf) ->
Result<Vec<PathBuf>, ImagixError> {
        let entries = fs::read_dir(src_folder)
        .map_err(|_e| ImagixError::UserInputError("Invalid source folder".to_string()))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap()
        .into_iter()
        .filter(|r| {
            r.extension() == Some("JPG".as_ref())
            || r.extension() == Some("jpg".as_ref())
            || r.extension() == Some("PNG".as_ref())
            || r.extension() == Some("png".as_ref())
        })
        .collect();


        Ok(entries)

}

fn resize_image(size: u32, src_folder: &mut PathBuf) -> Result<(), ImagixError> {
    let new_file_name = src_folder
        .file_stem()
        .unwrap()
        .to_str()
        .ok_or(io::ErrorKind::InvalidInput)
        .map(|f| format!("{}.png", f))
        .unwrap();

    let mut dest_folder = src_folder.clone();
    dest_folder.pop();
    dest_folder.push("tmp/");

    if !dest_folder.exists() {
        fs::create_dir(&dest_folder).unwrap();
    }

    dest_folder.pop();
    dest_folder.push("tmp/tmp.png");
    dest_folder.set_file_name(new_file_name.as_str());

    let timer = Instant::now();

    let img = image::open(&src_folder).unwrap();
    let scaled = img.thumbnail(size, size);
    
    let mut output = fs::File::create(&dest_folder).unwrap();
    scaled.write_to(&mut output, image::ImageFormat::Png).unwrap();

    let elapsed = timer.elapsed().as_millis();

    println!(
        "Thumbnailed file: {:?} to size {}x{} in {}. Output
        file
        in {:?}",
        src_folder,
        size,
        size,
        elapsed,
        dest_folder
        );

    Ok(())
}


//  Tests come here
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_single_image_resize() {
        let mut path = PathBuf::from("images/image1.jpg");
        let destination_path = PathBuf::from("images/tmp/image1.png");

        match process_resize_request(SizeOption::Small, Mode::Single, &mut path) {
            Ok(_) => println!("Successful resize of single image"),
            Err(e) => println!("Error in single image: {:?}", e),
        }

        assert_eq!(true, destination_path.exists())
    }

    #[test]
    fn test_multiple_image_resize() {
        let mut path = PathBuf::from("images/");
        let _res = process_resize_request(SizeOption::Small, Mode::All, &mut path);
        
        let destination_path1 = PathBuf::from("images/tmp/image1.png");
        let destination_path2 = PathBuf::from("images/tmp/image2.png");

        assert_eq!(true, destination_path1.exists());
        assert_eq!(true, destination_path2.exists());
    }
}
