use spicedb_grpc::authzed::api::v1::{ReadSchemaResponse, WriteSchemaResponse, ZedToken};

pub trait ReadSchemaResponseReader {
    fn schema_text(&self) -> &str;

    fn read_at(&self) -> Option<&str>;
}

impl ReadSchemaResponseReader for ReadSchemaResponse {
    fn schema_text(&self) -> &str {
        self.schema_text.as_str()
    }

    fn read_at(&self) -> Option<&str> {
        self.read_at.as_ref().map(ZedTokenReader::token)
    }
}

pub trait WriteSchemaResponseReader {
    fn written_at(&self) -> Option<&str>;
}

impl WriteSchemaResponseReader for WriteSchemaResponse {
    fn written_at(&self) -> Option<&str> {
        self.written_at.as_ref().map(ZedTokenReader::token)
    }
}

pub trait ZedTokenReader {
    fn token(&self) -> &str;
}

impl ZedTokenReader for ZedToken {
    fn token(&self) -> &str {
        &self.token
    }
}
