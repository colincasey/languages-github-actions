use crate::commands::BuilderFileError;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

pub(crate) enum Error {
    GetCurrentDir(std::io::Error),
    InvalidBuildpackUri(String, uriparse::URIReferenceError),
    InvalidBuildpackVersion(String, libcnb_data::buildpack::BuildpackVersionError),
    BuilderFile(BuilderFileError),
    WritingBuilder(PathBuf, std::io::Error),
    NoBuilderFiles(Vec<String>),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::GetCurrentDir(error) => {
                write!(f, "Could not get the current directory\nError: {error}")
            }

            Error::InvalidBuildpackUri(value, error) => {
                write!(
                    f,
                    "The buildpack URI argument is invalid\nValue: {value}\nError: {error}"
                )
            }

            Error::InvalidBuildpackVersion(value, error) => {
                write!(
                    f,
                    "The buildpack version argument is invalid\nValue: {value}\nError: {error}"
                )
            }

            Error::BuilderFile(builder_file_error) => match builder_file_error {
                BuilderFileError::Reading(path, error) => {
                    write!(
                        f,
                        "Could not read builder\nPath: {}\nError: {error}",
                        path.display()
                    )
                }

                BuilderFileError::Parsing(path, error) => {
                    write!(
                        f,
                        "Could not parse builder\nPath: {}\nError: {error}",
                        path.display()
                    )
                }
            },

            Error::WritingBuilder(path, error) => {
                write!(
                    f,
                    "Error writing builder\nPath: {}\nError: {error}",
                    path.display()
                )
            }

            Error::NoBuilderFiles(builders) => {
                write!(
                    f,
                    "No builder.toml files found in the given builder directories\n{}",
                    builders
                        .into_iter()
                        .map(|builder| format!("• {builder}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
        }
    }
}
