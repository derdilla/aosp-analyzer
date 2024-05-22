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
    pub fn new(file_name: &str) -> Language {
        let extension = file_name
            .split(".")
            .last()
            .expect("split always has last");
        // No lower case possible because c and cpp
        match extension {
            "java" => Language::JAVA,
            "kt" => Language::KOTLIN,
            "c" | "h" => Language::C,
            "cpp" | "CPP" | "cc" | "c++" | "cxx" | "CXX" | "hpp"
                | "hxx" | "Hxx" | "HXX" | "C" | "H" => Language::CPP,
            "rs" => Language::RUST,
            "py" | "py3" | "pxd" | "pyi" | "pyz" | "pywz" | "ipynb" => Language::PYTHON,
            "gradle" => Language::GRADLE,
            "cmake" => Language::CMAKE,
            "mk" | "MK" | "makefile" | "MAKEFILE" => Language::MAKEFILE,
            "s" | "S" | "asm" => Language::ASSEMBLY,
            _ => OTHER(extension.to_string())
        }
    }
}

// File extension resources:
// - https://www.openoffice.org/dev_docs/source/file_extensions.html
// - https://dcjtech.info/topic/python-file-extensions/
