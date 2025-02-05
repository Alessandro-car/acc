use std::fs::{self, File};
use std::io::{self, BufRead, Seek, SeekFrom, Write, Read};
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};

fn read_from_position(file_path: &str, position: u64, length: usize) -> io::Result<String> {
    let mut file = File::open(file_path).expect("Should have been able to open the file");
    file.seek(SeekFrom::Start(position));

    let mut buffer = vec![0; length];
    file.read_exact(&mut buffer);

    String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

#[derive(Debug)]
pub struct Preprocessor {
    include_paths: Vec<PathBuf>,
    contents_to_write: String,
    new_file: File,
    pos_in_file: u64,
}

impl Preprocessor {
    pub fn new(file_path: &str) -> Self {
        let preprocessed_file_path = "src/preprocessed.i";
        let new_file = File::create(preprocessed_file_path).expect("Should have been able to create the file");
        Preprocessor {
            include_paths: Vec::new(),
            contents_to_write: String::new(),
            new_file: new_file,
            pos_in_file: 0,
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
        //let file_metadata = fs::metadata(file_path).expect("Should have been able to get the metadata of the file");
        //let file_length = file_metadata.len() as usize;
        let mut reader = io::BufReader::new(file_to_preprocess);
        let mut line = String::new();
        while let Ok(bytes_read) = reader.read_line(&mut line) {
            if bytes_read == 0 {
                break;
            }
           // let current_pos = reader.seek(SeekFrom::Current(0)).expect("Could not get the current position");
            //self.pos_in_file = current_pos - (line.len() as u64);
            self.contents_to_write.push_str(line.as_str());
            //println!("{:?}", self.contents_to_write);
            self.process_line(line.as_str().trim());
            /*match read_from_position(file_path, current_pos, file_length) {
                Ok(content) => self.new_file.write(content.trim_matches(char::from(0)).as_bytes()),
                Err(e) => Err(e),
            };*/
            line.clear();
        }
        self.new_file.write(self.contents_to_write.as_bytes());
    }
}




