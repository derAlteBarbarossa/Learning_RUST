use std::ffi::OsStr;
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use super::errors::StatsError;

#[derive(Debug)]
pub struct ScrStats {
    pub number_of_files:    u32,
    pub line_of_code:       u32,
    pub comments:           u32,
    pub blanks:             u32,
}


pub fn get_src_stats_for_file(in_dir: &Path) -> 
    Result<ScrStats, StatsError> {

    let mut loc         = 0;
    let mut comments    = 0;
    let mut blanks      = 0;

    let mut multi_line_comment = false;

    let file_content = fs::read_to_string(in_dir)?;

    for line in file_content.lines() {
        if !multi_line_comment {
            if line.len() == 0 {
                blanks += 1;
            } else if line.trim_start().starts_with("//") {
                comments += 1;
            } else if line.contains("/*") && line.contains("/*") {
                comments += 1;
            } else if line.contains("/*") {
                comments += 1;
                multi_line_comment = true;
            } else {
                loc += 1;
            }
        } else {
            comments += 1;
            if line.contains("*/") && !line.contains("/*") {
                multi_line_comment = false;
            }
        }
    }

    Ok(ScrStats {
        number_of_files: 0,
        line_of_code: loc,
        comments: comments,
        blanks: blanks,
    })

}

pub fn get_summary_src_stats(in_dir: &Path) -> 
    Result<ScrStats, StatsError> {

    let mut total_line_of_codes =   0;
    let mut total_comments =        0;
    let mut total_blanks =          0;

    let mut dir_entries: Vec<PathBuf> = vec![in_dir.to_path_buf()];
    let mut file_entries: Vec<DirEntry> = vec![];

    while let Some(entry) = dir_entries.pop() {
        for inner_entry in fs::read_dir(entry)? {
            if let Ok(entry) = inner_entry {
                if entry.path().is_dir() {
                    dir_entries.push(entry.path())
                } else {
                    if entry.path().extension() == Some(OsStr::new("rs")) {
                        file_entries.push(entry)
                    }
                }
            }
        }
    }

    let total_number_of_files = file_entries.len();

    for entry in file_entries {
        let stat: ScrStats = get_src_stats_for_file(&entry.path()).unwrap();

        total_line_of_codes += stat.line_of_code;
        total_comments      += stat.comments;
        total_blanks        += stat.blanks;
    }

    Ok(ScrStats {
        number_of_files:    total_number_of_files as u32,
        line_of_code:       total_line_of_codes,
        comments:           total_comments,
        blanks:             total_blanks,   
    })

}