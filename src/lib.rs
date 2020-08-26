/*  SCGSM Rust Edition - A simple specification for managing save data
 *  Copyright (C) 2020 Spencer Smith <spenny@geniuspiece.com>
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::str::FromStr;
use std::fmt::Debug;

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

pub struct SCGSM {
    pub flags: Vec<Flag>,
}

impl SCGSM {
    pub fn init() -> SCGSM {
        SCGSM {
            flags: Vec::new()
        }
    }

    pub fn add_flag(&mut self, k: &str, v: &str) {
        self.flags.push(Flag::init(k, v));
    }

    pub fn cast_value<T>(&mut self, i: usize) -> T where T: FromStr, <T as FromStr>::Err: Debug {
        self.flags[i].value.parse::<T>().unwrap()
    }

    pub fn load_flags(&mut self, sf: &str) {
        let savefile = std::fs::read_to_string(sf).expect("File not found!"); 

        for line in savefile.lines() {
            let parse_line: Vec<&str> = line.split('|').collect();
            if sf == "baseflags.scgs" { self.add_flag(parse_line[0], parse_line[1]); }
            else { self.update_flag(parse_line[0], parse_line[1]); }
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
