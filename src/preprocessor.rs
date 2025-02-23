use std::fs::{self, File};
use std::io::{self, BufRead, Seek, SeekFrom, Write, Read};
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Preprocessor {
    include_paths: Vec<PathBuf>,
    contents_to_write: String,
    new_file: File,
}

impl Preprocessor {
    pub fn new(file_path: &str) -> Self {
        let preprocessed_file_path = "src/preprocessed.i";
        let new_file = File::create(preprocessed_file_path).expect("Should have been able to create the file");
        Preprocessor {
            include_paths: Vec::new(),
            contents_to_write: String::new(),
            new_file: new_file,
        }
    }

    pub fn add_include_paths<P: AsRef<Path>>(&mut self, path: P) {
        self.include_paths.push(path.as_ref().to_owned());
    }

    fn process_line(&mut self, line: &str) {
        if line.starts_with('#') {
            self.handle_directive(line);
        }
    }

    fn handle_directive(&mut self, line: &str) {
        let mut parts = line.split_whitespace();
        let directive = parts.next().unwrap_or("").trim_start_matches('#');

        match directive {
            "include" => {
                let filename = parts.next().expect("Should have been able to read the filename").trim_matches(|c| c == '"' || c == '<' || c == '>');
                self.include_file(filename);
            }
            _ => {},
        }
    }

    fn include_file(&mut self, filename: &str) {
        for include_path in &self.include_paths {
            let full_path = include_path.join(filename);
            if full_path.exists() {
                let included_contents = fs::read_to_string(&full_path).unwrap();
                let start_position_to_replace = self.contents_to_write.find("#include").unwrap();
                self.contents_to_write.replace_range(start_position_to_replace.., included_contents.as_str());
            }
        }
    }

    pub fn process_file(&mut self, file_path: &str) {
        self.add_include_paths("src/");
        let mut file_to_preprocess = File::open(file_path).expect("Should have been able to open the file");
        let mut reader = io::BufReader::new(file_to_preprocess);
        let mut line = String::new();
        while let Ok(bytes_read) = reader.read_line(&mut line) {
            if bytes_read == 0 {
                break;
            }
            self.contents_to_write.push_str(line.as_str());
            self.process_line(line.as_str().trim());
            line.clear();
        }
        self.new_file.write(self.contents_to_write.as_bytes());
    }
}
