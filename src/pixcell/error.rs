use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum TextureLoadError 
{
    IoError(std::io::Error),
	UnsupportedFileType(String),
	CorruptData(String),
	CorruptHeader(String),
    HeaderParseError(String),
}

impl fmt::Display for TextureLoadError 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
	{
        match self 
		{
            TextureLoadError::IoError(err) => { write!(f, "IO Error: {}", err) },
            TextureLoadError::UnsupportedFileType(file_type) => { write!(f, "Unsupported file type: '{}'", file_type) },
            TextureLoadError::CorruptData(err) => { write!(f, "Corrupt Data in image : '{}'", err) },
            TextureLoadError::CorruptHeader(err) => { write!(f, "Corrupt Header in image : '{}'", err) },
            TextureLoadError::HeaderParseError(err) => { write!(f, "Failed to parse header : '{}'", err) },
        }
    }
}

impl Error for TextureLoadError {}

impl From<std::io::Error> for TextureLoadError 
{
    fn from(err: std::io::Error) -> TextureLoadError 
	{
        TextureLoadError::IoError(err)
    }
}
