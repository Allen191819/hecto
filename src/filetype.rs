pub struct FileType {
    name: String,
    hl_opts: HighlighterOptions,
}
#[derive(Default)]
pub struct HighlighterOptions {
    pub numbers: bool,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_opts: HighlighterOptions::default(),
        }
    }
}

impl FileType {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn from(file_name:&str)->Self{
        if file_name.ends_with(".rs"){
            return Self{
                name:String::from("Rust"),
                hl_opts:HighlighterOptions { numbers: true }
            };
        }
        Self::default()
    }
}
