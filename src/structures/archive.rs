#[derive(Debug, PartialEq)]
pub struct Archive {
    pub or: String,
    pub items: Vec<ArchiveItem>,
}

#[derive(Debug, PartialEq)]
pub struct ArchiveItem {
    pub res: String,
    pub name: String,
}
