use std::num::ParseIntError;

use thiserror::Error;

use crate::fs::{File, FileEntry, FileSystem, MovementError};

enum ParsingResult<'a> {
    FileEntry(FileEntry<'a>),
    MoveDown(&'a str),
    MoveUp,
    DoNothing,
}

pub fn build_file_system(contents: &str) -> Result<FileSystem, ExecutionError> {
    let parsed_lines = contents.lines().skip(1).map(parse_line);

    let mut fs = FileSystem::default();
    for parsed_line in parsed_lines {
        let parsed_line = parsed_line?;

        match parsed_line {
            ParsingResult::FileEntry(file_entry) => {
                let new_node = fs.fs.add_node(file_entry);
                fs.fs.add_edge(fs.current_path, new_node, ());
            }
            ParsingResult::MoveDown(name) => fs.move_down(name)?,
            ParsingResult::MoveUp => fs.move_up()?,
            ParsingResult::DoNothing => (),
        }
    }

    Ok(fs)
}

fn parse_line(line: &str) -> Result<ParsingResult, ParseError> {
    let line: Vec<&str> = line.split(' ').collect();

    match line[..] {
        ["dir", folder_name] => Ok(ParsingResult::FileEntry(FileEntry::Folder(folder_name))),
        ["$", "cd", ".."] => Ok(ParsingResult::MoveUp),
        ["$", "cd", name] => Ok(ParsingResult::MoveDown(name)),
        ["$", "ls"] => Ok(ParsingResult::DoNothing),
        [size, name] => Ok(ParsingResult::FileEntry(FileEntry::File(File {
            size: size.parse()?,
            name,
        }))),
        _ => Err(ParseError::SyntaxError),
    }
}

#[derive(Debug, Error)]
#[error("Parse error")]
pub enum ParseError {
    ParseIntError(#[from] ParseIntError),
    SyntaxError,
}

#[derive(Debug, Error)]
#[error("Execution error")]
pub enum ExecutionError {
    ParseError(#[from] ParseError),
    MovementError(#[from] MovementError),
}
