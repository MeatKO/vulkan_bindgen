use std::{fmt, error::Error};

#[derive(Debug)]
pub enum ModelLoadError 
{
    IoError(std::io::Error),
	UnsupportedFileType,
    ParseError,
    UnexpectedTokenCount,
    UnknownToken,
	MaterialFileNotFound(String, std::io::Error),
	MaterialNotFound(String),
	MaterialWithoutMaps(String),
}

impl fmt::Display for ModelLoadError 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
	{
        match self 
		{
            ModelLoadError::IoError(err) => { write!(f, "IO Error: {}", err) },
            ModelLoadError::UnsupportedFileType => { write!(f, "Unsupported file type") },
            ModelLoadError::ParseError => { write!(f, "Failed to parse a value") },
            ModelLoadError::UnexpectedTokenCount => { write!(f, "Unexpected token count") },
            ModelLoadError::UnknownToken => { write!(f, "Unknown token encountered") },
            ModelLoadError::MaterialFileNotFound(mtl_name, err) => { write!(f, "Couldn't find material file '{}' : {}", mtl_name, err) },
            ModelLoadError::MaterialNotFound(mtl_name) => { write!(f, "Couldn't find material '{}'", mtl_name) },
            ModelLoadError::MaterialWithoutMaps(mtl_name) => { write!(f, "Couldn't parse material with no maps '{}'", mtl_name) },
        }
    }
}

impl Error for ModelLoadError {}

impl From<std::io::Error> for ModelLoadError 
{
    fn from(err: std::io::Error) -> ModelLoadError 
	{
        ModelLoadError::IoError(err)
    }
}
