use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use super::LoggerConfig;
use crate::encode::parser::Message;
use crate::runtime::append::Appender;

#[derive(Debug)]
pub struct Material {
    pub lc: LoggerConfig,
}

fn seg_size(dst: &String) -> u64 {
    let exist = Path::new(dst.as_str()).exists();
    if !exist {
        return 0;
    }

    let x = fs::metadata(dst.as_str()).unwrap().len();
    x
}

fn seg_rename(src: &String, dst: &String) -> std::io::Result<()> {
    let exist = Path::new(src.as_str()).exists();
    if !exist {
        return Ok(());
    }

    fs::rename(src.as_str(), dst.as_str())
}

fn seg_remove(dst: &String) -> std::io::Result<()> {
    let exist = Path::new(dst.as_str()).exists();
    if !exist {
        return Ok(());
    }

    fs::remove_file(dst.as_str())
}

fn segs_count(file_path: &String) -> Vec<String> {
    let path = Path::new(file_path.as_str());
    let parent = path.parent();
    let log_dir = parent.unwrap().display().to_string();

    let paths = fs::read_dir(log_dir.as_str()).unwrap();
    let l = file_path.len();
    let pattern = file_path.as_str();

    let mut segs: Vec<String> = vec![];
    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_dir() {
            let s = path.display().to_string();
            let d = s.get(..l);
            match d {
                None => {}
                Some(d) => match pattern.find(d) {
                    None => {}
                    Some(_v) => {
                        segs.push(s);
                    }
                },
            }
        }
    }

    segs
}

fn segs_rotate(dst: &String, segs: &mut Vec<String>, count: usize, rename: bool) {
    let tot_len = segs.len();
    if tot_len == 0 {
        return;
    }

    segs.reverse();
    let del_len = tot_len - count;

    if del_len > 0 {
        for i in 0..del_len {
            match seg_remove(&segs[i]) {
                Ok(_v) => {}
                Err(e) => {
                    println!("segs_rotate_slow remove failed {:?}", e);
                }
            }
        }
    }

    if rename {
        for i in del_len..tot_len {
            let new_name = format!("{}.{}", dst, tot_len - i);
            match seg_rename(&segs[i].to_string(), &new_name) {
                Ok(_v) => {}
                Err(e) => {
                    println!("segs_rotate_slow rename failed {:?}", e);
                }
            }
        }
    }
}

impl Material {
    pub fn new(conf: LoggerConfig) -> Material {
        Material { lc: conf }
    }

    fn rotate(&self, current_len: usize) -> std::io::Result<()> {
        let mut tot: usize = 0;
        let exist = Path::new(self.lc.path.as_str()).exists();
        if exist {
            let seg_sz = seg_size(&self.lc.path);
            tot += seg_sz as usize + current_len;
        }

        let mut segs = segs_count(&self.lc.path);
        let seg_count = segs.len();

        if tot <= self.lc.max_size as usize && seg_count < self.lc.max_segments {
            return Ok(());
        }

        if tot > self.lc.max_size as usize {
            segs_rotate(&self.lc.path, &mut segs, self.lc.max_segments - 1, true);
        } else {
            if exist {
                segs_rotate(&self.lc.path, &mut segs, self.lc.max_segments, false);
            } else {
                segs_rotate(&self.lc.path, &mut segs, self.lc.max_segments - 1, false);
            }
        }

        Ok(())
    }

    fn output(&self, msg: &Message) {
        let data = format!(
            "{} {} {}\n",
            chrono::offset::Local::now().to_string(),
            msg.level.to_string(),
            msg.args
        );

        match self.rotate(data.len()) {
            Ok(_v) => {}
            Err(e) => {
                println!("rotate failed {:?}", e);
            }
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.lc.path.as_str())
            .unwrap();
        file.write(data.as_bytes()).unwrap();
    }
}

impl Appender for Material {
    fn append(&self, msg: &Message) -> Result<(), String> {
        self.output(msg);
        Ok(())
    }
}
