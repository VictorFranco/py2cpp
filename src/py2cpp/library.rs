use crate::py2cpp::types::Library;

impl Library {

    pub fn get_libraries(names: &[&str]) -> Vec<Library> {
        let mut libraries = Vec::new();
        for name in names.iter() {
            let name = name.to_string();
            libraries.push(
                Library { name }
            );
        }
        libraries
    }

}
