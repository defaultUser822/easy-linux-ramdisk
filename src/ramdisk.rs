use std::fs::File;
use std::io::{BufRead, BufReader};

// TODO: Replace `(String, String)` with a specialized struct
// TODO: Remove `.unwrap`

fn get_mounts() -> Vec<(String, String)> {
    let file = File::open("/proc/mounts").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut result: Vec<(String, String)> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let processed_line: Vec<&str> = line.split(' ').collect();
        result.push((processed_line[1].to_string(), processed_line[2].to_string()));
    }
    result
}

pub fn get_tmpfs_mounts() -> Vec<(String, String)> {
    let mounts = get_mounts();
    let mut result: Vec<(String, String)> = Vec::new();

    for (location, filesystem) in mounts {
        if filesystem == "tmpfs" {
            result.push((location, filesystem));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_shows_tmpfs_mounts() {
        let mounts = get_tmpfs_mounts();

        for mount in mounts {
            assert_eq!(mount.1, "tmpfs");
        }
    }
}
