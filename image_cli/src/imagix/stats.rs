use std::path::PathBuf;

use super::error::*;
use super::resize::*;

pub fn get_stats (src_folder: PathBuf) -> Result<(usize, f64), ImagixError> {

    let image_files = get_image_files(src_folder.to_path_buf())?;

    let size = image_files
            .iter()
            .map(|f| f.metadata().unwrap().len())
            .sum::<u64>();
    
    Ok((image_files.len(), (size / 1000000) as f64))
}