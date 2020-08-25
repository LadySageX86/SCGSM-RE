/*****************************************************
 *              SCFS - Rust Edition                  *
 *      A simple standard for managing save data     *
 * Written by Spencer Smith (spenny@geniuspiece.com) *
 *****************************************************/

pub struct Flag {
    pub key: String,
    pub value: String
}

impl Flag {
    pub fn init(k: &str, v: &str) -> Flag {
        Flag {
            key: String::from(k),
            value: String::from(v)
        }
    }
}

pub struct SCFS {
    pub flags: Vec<Flag>,
}

impl SCFS {
    pub fn init() -> SCFS {
        SCFS {
            flags: Vec::new()
        }
    }

    pub fn add_flag(&mut self, k: &str, v: &str) {
        self.flags.push(Flag::init(k, v));
    }

    pub fn load_flags(&mut self, sf: &str) {
        let savefile = std::fs::read_to_string(sf).expect("File not found!"); 

        for line in savefile.lines() {
            let parse_line: Vec<&str> = line.split('|').collect();
            self.add_flag(parse_line[0], parse_line[1]);
        }
    }

    pub fn save_flags(&mut self, sf: &str) -> std::io::Result<()> {
        std::fs::write(sf, self.view_flags().as_str())?;
        Ok(())
    }

    pub fn update_flag(&mut self, k: &str, v: &str) {
        for f in self.flags.iter_mut() {
            if f.key == String::from(k) {
                f.value = String::from(v);
            }
        }
    }

    pub fn view_flags(&mut self) -> String {
        let mut fs = String::new();
        for f in self.flags.iter() {
            fs.push_str(f.key.as_str());
            fs.push('|');
            fs.push_str(f.value.as_str());
            fs.push('\n');
        }
        fs
    }
}
