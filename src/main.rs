use std::{env, fs, io, path, thread};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;

fn main() {
    env::set_current_dir("/home/derdilla/android-source/").unwrap();
    const THREAD_COUNT: u8 = 12;

    // create threads
    let lists = load_index(THREAD_COUNT as usize);
    let (tx, rx) = mpsc::channel();
    for thread_files in lists {
        let tx = tx.clone();
        thread::spawn(move || {
            println!("Starting thread...");
            let mut data: HashMap<String, u128> = HashMap::new();
            for file in thread_files {
                let file_type = get_filetype(file.as_str())
                    .unwrap_or("unknown");
                if ["jar", "so"].iter().any(|e| (&file_type).ends_with(e)) {
                    //println!("Skipping uncountable file type.");
                    continue;
                }
                match count_lines(&file) {
                    Ok(line_count) => {
                        let mut line_count: u128 = u128::try_from(line_count)
                            .expect("line count should be <= 0");
                        line_count += **(&data.get(&file_type.to_string()).unwrap_or(&0));
                        data.insert(file_type.to_string(), line_count);
                    }
                    Err(_) => {
                        eprintln!("Can't count lines in file {file}")
                    }
                }
            }
            tx.send(data).expect("rx should live long enough");
        });
    }

    // count data
    let mut data: HashMap<String, u128> = HashMap::new();
    let mut finished_count = 0;
    for result in rx {
        finished_count += 1;
        println!("Thread {} finished.", &finished_count);

        data.extend(result);
        println!("Fetched data:\n{:?}", data);
        if finished_count >= THREAD_COUNT {
            break;
        }
    }
    println!("All threads finished!");
}

/// Loads relative paths from the files.index file in the current working directory into multiple
/// vectors. [parts] specifies the amount of vectors.
fn load_index(parts: usize) -> Vec<Vec<String>> {
    let text = fs::read_to_string("files.index").unwrap();
    let mut lines = text.lines();

    let count = &lines.clone().count();
    let step: usize = count / parts;
    println!("Loaded {} files from index.", count);

    let mut lists = Vec::new();
    for i in 0..parts {
        let step = if i == (parts - 1) { // include last entries
            count - (step * i)
        } else {
            count / parts
        } ;

        let mut l = Vec::new();
        for _ in 0..step {
            l.push((&mut lines).next().unwrap().to_owned())
        }
        lists.push(l);
    }

    lists
}

pub fn count_lines<P: AsRef<path::Path>>(path: P) -> Result<usize, io::Error> {
    let handle = fs::File::open(path).unwrap();
    let mut reader = BufReader::with_capacity(1024 * 32, handle);
    let mut count = 0;
    loop {
        let len = {
            let buf = reader.fill_buf()?;
            if buf.is_empty() {
                break;
            }
            count += bytecount::count(&buf, b'\n');
            buf.len()
        };
        reader.consume(len);
    }
    Ok(count)
}

pub fn get_filetype(path: &str) -> Option<&str> {
    path::Path::new(path)
        .extension()?
        .to_str()
}
