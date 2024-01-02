use std::{env, fs, io, path, process, thread};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use regex::Regex;

const ANDROID_SOURCE: &str = "/home/derdilla/android-source/aosp14/";

/// Beginning of path names that are ignored.
const EXCLUDED_DIRS: [&str; 22] = ["./.repo", "./tools", "./prebuilt", "./out", "./external",
    "./device", "./kernel/prebuilts", "./test/catbox/prebuilts", "./developers/build/prebuilts",
    "./developers/samples/android/prebuilts", "./developers/samples/android/prebuilts",
    "./development/vndk/tools/header-checker/tests/integration/version_script_example/prebuilts",
    "./development/gki/kmi_abi_chk/prebuilts", "./development/host/windows/prebuilt",
    "./system/teeui/libteeui/prebuilt", "./system/apex/tests/testdata/sharedlibs/prebuilts",
    "./system/apex/shim/prebuilts", "./frameworks/libs/binary_translation/prebuilt",
    "./frameworks/opt/car/services/builtInServices/prebuilts",
    "./cts/hostsidetests/packagemanager/extractnativelibs/res/prebuilt",
    "./packages/modules/Virtualization/libs/apkverify/tests/data/input.272629760",
    "./cts/tests/tests/packageinstaller/atomicinstall/testdata/apk/prebuilt"];
const EXCLUDED_TYPES: [&str; 16] = ["jar", "so", "obj", "webp", "class", "jpeg", "exe", "webm",
    "mp4", "apk", "apex", "ko", "lz4", "gz", "debug", "cr2", ];
const THREAD_COUNT: usize = 12;

// TODO: consider counting ./packages differently

fn main() {
    env::set_current_dir(ANDROID_SOURCE).unwrap();

    // create threads
    let lists = load_index();
    let (tx, rx) = mpsc::channel();
    for thread_files in lists {
        let tx = tx.clone();
        thread::spawn(move || {
            //println!("Starting thread...");
            let mut data: HashMap<String, u128> = HashMap::new();
            analyze_file_list(&mut data, thread_files);
            tx.send(data).expect("rx should live long enough");
        });
    }

    // count data
    let mut data: HashMap<String, u128> = HashMap::new();
    let mut finished_count = 0;
    for result in rx {
        finished_count += 1;
        //println!("Thread {} finished.", &finished_count);

        data.extend(result);
        // remove outliers and empty files:
        data.retain(|_, value| value > &mut 1);
        //println!("Fetched data:\n{:?}", data);
        if finished_count >= THREAD_COUNT {
            break;
        }
    }

    println!("Key stats:\n
\t.java\t{}
\t.kt  \t{}
\t.cpp \t{}
\t.c   \t{}
\t.rs  \t{}
\t.py  \t{}
\t.glsl\t{}
\t.gradle\t{}
\t.cmake\t{}",
        data.get("java").unwrap_or(&0),
        data.get("kt").unwrap_or(&0),
        data.get("cpp").unwrap_or(&0),
        data.get("c").unwrap_or(&0),
        data.get("rs").unwrap_or(&0),
        data.get("py").unwrap_or(&0),
        data.get("glsl").unwrap_or(&0),
        data.get("gradle").unwrap_or(&0),
        data.get("cmake").unwrap_or(&0),
    );
    // sum of lines
    println!("\nTotal:\t{}", data.get("java").unwrap_or(&0) +
             data.get("kt").unwrap_or(&0) +
             data.get("cpp").unwrap_or(&0) +
             data.get("c").unwrap_or(&0) +
             data.get("rs").unwrap_or(&0) +
             data.get("py").unwrap_or(&0) +
             data.get("glsl").unwrap_or(&0)+
             data.get("gradle").unwrap_or(&0)+
             data.get("cmake").unwrap_or(&0));
}

/// Loads relative paths from the files.index file in the current working directory into multiple
/// vectors. [parts] specifies the amount of vectors.
fn load_index() -> Vec<Vec<String>> {
    let text = fs::read_to_string("files.index").or_else(|_| -> io::Result<String> {
        eprintln!("No index created. Run the following in the android source directory:\n\tfind . -type f -name \"*\" > files.index");
        process::exit(1);
    }).expect("Error handled.");
    let mut lines = text.lines()
        .filter(|path|
            EXCLUDED_DIRS.iter().all(|dir_name|
                !path.starts_with(dir_name)
            )
        );

    let count = &lines.clone().count();
    let step: usize = count / THREAD_COUNT;
    println!("Loaded {} files from index.", count);

    let mut lists = Vec::new();
    for i in 0..THREAD_COUNT {
        let step = if i == (THREAD_COUNT - 1) { // include last entries
            count - (step * i)
        } else {
            count / THREAD_COUNT
        } ;

        let mut l = Vec::new();
        for _ in 0..step {
            l.push((&mut lines).next().unwrap().to_owned())
        }
        lists.push(l);
    }

    lists
}

/// Return the paths from files.index which meet any of the [conditions]. If [is_blacklist] is set
/// all other files are returned.
fn load_special_index(conditions: Vec<Regex>, is_blacklist: bool) -> Vec<String> {
    let text = fs::read_to_string("files.index").or_else(|_| -> io::Result<String> {
        eprintln!("No index created. Run the following in the android source directory:\n\tfind . -type f -name \"*\" > files.index");
        process::exit(1);
    }).expect("Error handled.");
    text.lines()
        .filter(|path| {
            let has_matches = conditions.iter().any(|regex|
                regex.is_match(path)
            );
            if is_blacklist { !has_matches } else { has_matches }
        })
        .collect()
    // TODO use this function for collecting information in folders like external, packages or tests to filter out and display separately
}

/// Count lines per file extension and save to data
pub fn analyze_file_list(data: &mut HashMap<String, u128>, files: Vec<String>) {
    for file in files {
        let file_type = get_filetype(file.as_str())
            .unwrap_or("unknown");
        if EXCLUDED_TYPES.iter().any(|e| (&file_type).ends_with(e)) {
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
}

/// Counts lines of code in a single files
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
