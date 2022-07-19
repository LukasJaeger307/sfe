/* 
 * Copyright 2022, Lukas Jäger
 *
 * This file is part of SFE.
 *
 * SFE is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * SFE is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with SFE.  If not, see <http://www.gnu.org/licenses/>.
 */
use memmap::Mmap;
use memmap::MmapMut;
use memmap::MmapOptions;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;

#[derive(Clone, Copy)]
pub enum Direction {
    Encrypt,
    Decrypt,
}

pub struct LoadedFiles {
    source : Mmap,
    destination : MmapMut,
    direction : Direction,
}

pub struct FileLoader {
    direction : Direction,
    filename_extension : String,
    header_size : usize,
}

impl FileLoader {
    pub fn load_files(&self, path : &String) -> Option<LoadedFiles> {
        match self.direction {
            Direction::Encrypt => self.load_files_for_encryption(path),
            Direction::Decrypt => self.load_files_for_decryption(path),
        }
    }

    fn load_files_for_encryption(&self, path : &String) -> Option<LoadedFiles> {
        if !Path::new(path).exists() {
            None
        } else {
            let ciphertext_path = path.to_owned() + &".".to_string() + &self.filename_extension.to_owned();
            if Path::new(&ciphertext_path).exists() {
                None
            } else {
                let source = match self.create_source_memmap(path) {
                    None => {return None;}
                    Some(source) => source
                };
                let destination_file = match OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(ciphertext_path) {
                    Ok(file) => file,
                    Err(err) => {
                        return None;
                    }
                };
                let source_len : usize = source.len();
                match destination_file.set_len((self.header_size + source_len) as u64) {
                    Ok(ok) => {},
                    Err(err) => {
                        return None;
                    }
                };
                let destination = match unsafe{MmapMut::map_mut(&destination_file)} {
                    Ok(destination) => destination,
                    Err(err) => {
                        return None;
                    }
                };
                Some(LoadedFiles{
                    source : source,
                    destination : destination,
                    direction : self.direction,
                })
            }
        }
    }

    fn create_source_memmap(&self, path : &String) -> Option<Mmap> {
        let source_file = match File::open(path) {
            Ok(file) => Some(file),
            Err(err) => None
        };
        match source_file {
            None => None,
            Some(source_file) => {
                match unsafe{MmapOptions::new().map(&source_file)} {
                    Ok(source) => Some(source),
                    Err(err) => None
                }
            }
        }
    }
    
    fn load_files_for_decryption(&self, path : &String) -> Option<LoadedFiles> {
        // TODO: Implement!
        None
    }

}


#[cfg(test)]
mod file_loader_tests {
    use super::*;

    #[test]
    fn test_load_files_nonexistent() {
        let loader : FileLoader = FileLoader{
            direction : Direction::Encrypt,
            filename_extension : "enc".to_string(),
            header_size : 28
        };
        let loaded_files = loader.load_files(&"Nonexistent.txt".to_string());
        assert!(loaded_files.is_none());
    }
    
    #[test]
    fn test_load_files_encryption_ciphertext_exists() {
        let loader : FileLoader = FileLoader{
            direction : Direction::Encrypt,
            filename_extension : "enc".to_string(),
            header_size : 28
        };
        let loaded_files = loader.load_files(&"testfiles/test1.txt".to_string());
        assert!(loaded_files.is_none());
    }
    
    #[test]
    fn test_load_files_decryption_plaintext_exists() {
        let loader : FileLoader = FileLoader{
            direction : Direction::Decrypt,
            filename_extension : "enc".to_string(),
            header_size : 28
        };
        let loaded_files = loader.load_files(&"testfiles/test1.txt.enc".to_string());
        assert!(loaded_files.is_none());
    }
    
    use std::fs;

    #[test]
    fn test_load_files_encryption() {
        if Path::new("testfiles/test2.txt.enc").exists() {
            match fs::remove_file("testfiles/test2.txt.enc") {
                Ok(ok) => {},
                Err(err) => {
                    assert!(false);
                }
            };
        }
        let loader : FileLoader = FileLoader{
            direction : Direction::Encrypt,
            filename_extension : "enc".to_string(),
            header_size : 28
        };
        let loaded_files = loader.load_files(&"testfiles/test2.txt".to_string());
        assert!(loaded_files.is_some());
        let loaded_files = loaded_files.unwrap();
        assert!(loaded_files.source.starts_with(b"test2.txt"));
        let source_len : usize = loaded_files.source.len();
        let destination_len : usize = loaded_files.destination.len();
        assert_eq!(loaded_files.destination.len(), source_len + loader.header_size);
    }
}
