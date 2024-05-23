use crate::language::Language::OTHER;

/// Common supported programming languages indicated by file extensions.
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Language {
    JAVA,
    KOTLIN,
    C,
    CPP,
    RUST,
    PYTHON,
    GRADLE,
    CMAKE,
    MAKEFILE,
    ASSEMBLY,
    OTHER(String),
    // TODO: toml, yaml, md ...
}

impl Language {
    /// Finds the language matching to the extension in the [file_name].
    pub fn new(file_name: &str) -> Option<Language> {
        let extension = file_name
            .split(".")
            .last()
            .expect("split always has last");
        // No lower case possible because c and cpp
        if extension.len() > 15 {
            return None;
        }
        match extension {
            "java" => Some(Language::JAVA),
            "kt" => Some(Language::KOTLIN),
            "c" | "h" => Some(Language::C),
            "cpp" | "CPP" | "cc" | "c++" | "cxx" | "CXX" | "hpp"
                | "hxx" | "Hxx" | "HXX" | "C" | "H" => Some(Language::CPP),
            "rs" => Some(Language::RUST),
            "py" | "py3" | "pxd" | "pyi" | "pyz" | "pywz" | "ipynb" => Some(Language::PYTHON),
            "gradle" => Some(Language::GRADLE),
            "cmake" => Some(Language::CMAKE),
            "mk" | "MK" | "makefile" | "MAKEFILE" => Some(Language::MAKEFILE),
            "s" | "S" | "asm" => Some(Language::ASSEMBLY),
            "jar" | "so" | "obj" | "webp" | "class" | "jpeg" | "exe" | "webm" |
            "db" | "original" | "iml" | "dex" | "sha1" | "ttf" | "aab" |
            "mp4" | "apk" | "apex" | "ko" | "lz4"| "gz"| "debug"| "cr2" => None,

            _ => Some(OTHER(extension.to_string()))
        }
    }
}

// File extension resources:
// - https://www.openoffice.org/dev_docs/source/file_extensions.html
// - https://dcjtech.info/topic/python-file-extensions/
