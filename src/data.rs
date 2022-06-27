

pub struct Matter {
    pub name: String,
    pub file_path: Option<String>,
    pub kholle_path: Option<String>
}


impl Matter {
    pub fn new(name: String) -> Self {
        Self {
            name, 
            file_path: None,
            kholle_path: None
        }
    }

    #[inline]
    pub fn set_file_path(&mut self, file_path: String) {
        self.file_path = Some(file_path)
    }

    #[inline]
    pub fn set_kholle_path(&mut self, kholle_path: String) {
        self.kholle_path = Some(kholle_path)
    }
}


pub enum MatterFolder {
    Folder(String, Vec<MatterFolder>), //folder_name content
    File(String, String) //filename url
}