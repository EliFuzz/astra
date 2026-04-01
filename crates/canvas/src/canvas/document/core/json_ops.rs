use super::types::CanvasDocument;

impl CanvasDocument {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn to_json_compact(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn to_writer<W: std::io::Write>(&self, writer: W) -> Result<(), serde_json::Error> {
        serde_json::to_writer(writer, self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let mut doc: Self = serde_json::from_str(json)?;
        doc.spatial_index.rebuild(&doc.shapes);
        Ok(doc)
    }

    pub fn from_reader<R: std::io::Read>(reader: R) -> Result<Self, serde_json::Error> {
        let mut doc: Self = serde_json::from_reader(reader)?;
        doc.spatial_index.rebuild(&doc.shapes);
        Ok(doc)
    }
}
