use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_mounts() -> Vec<(String, String)> {
    let file = File::open("/proc/mounts").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut result: Vec<(String, String)> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let processed_line: Vec<&str> = line.split(' ').collect();
        result.push((processed_line[0].to_string(), processed_line[1].to_string()));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_mounts_test() {
        let mounts = get_mounts();

        assert_eq!(mounts[0].0, "/dev/nvme0n1p5".to_string());
        assert_eq!(mounts[2].0, "tmpfs".to_string());
    }
}
