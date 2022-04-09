use std::{fs::OpenOptions, path::Path};

use syn::{File, Item};

mod error;

pub mod enum_hdr;
pub mod enum_impl;
pub mod names;
pub mod types;

use error::{Error, Result};

use self::names::IdentName;

pub struct Generator {
    file: File,
    namespace: Vec<String>,
    header_dir: String,
    source_dir: String,
}

impl Generator {
    pub fn builder_with<'a>(source_code: &'a str) -> Builder<'a> {
        Builder::new(source_code)
    }

    pub fn generate(&self) -> Result<()> {
        self.generate_enum_headers()?;
        self.generate_enum_sources()?;

        Ok(())
    }

    fn generate_enum_headers(&self) -> Result<()> {
        let header_path = Path::new(&self.header_dir);
        let namespace_path = header_path.join(self.namespace.join("/"));

        std::fs::create_dir_all(&namespace_path)?;

        for item in self.file.items.iter() {
            if let Item::Enum(enum_item) = item {
                let enum_ident: IdentName = (&enum_item.ident).into();

                let type_header_path =
                    namespace_path.join(format!("{}.hpp", enum_ident.to_file_name()));
                let type_inc_path =
                    namespace_path.join(format!("{}.inc.hpp", enum_ident.to_file_name()));
                let type_fmt_path =
                    namespace_path.join(format!("{}.fmt.hpp", enum_ident.to_file_name()));

                // generate type header
                if type_header_path.exists() {
                    println!("Skipping {}...", type_header_path.to_str().unwrap());
                } else {
                    println!("Generating {}...", type_header_path.to_str().unwrap());
                    let mut type_header_file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(type_header_path)?;
                    enum_hdr::write(&mut type_header_file, enum_item, &self.namespace)?;
                }

                // generate inc header
                println!("Generating {}...", type_inc_path.to_str().unwrap());
                {
                    if type_inc_path.exists() {
                        std::fs::remove_file(&type_inc_path)?;
                    }

                    let mut type_inc_file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(type_inc_path)?;
                    enum_hdr::write_inc(&mut type_inc_file, enum_item)?;
                }

                // generate fmt headers
                println!("Generating {}...", type_fmt_path.to_str().unwrap());
                {
                    if type_fmt_path.exists() {
                        std::fs::remove_file(&type_fmt_path)?;
                    }

                    let mut type_fmt_file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(type_fmt_path)?;
                    enum_hdr::write_fmt(&mut type_fmt_file, enum_item, &self.namespace)?;
                }
            }
        }

        Ok(())
    }

    fn generate_enum_sources(&self) -> Result<()> {
        let source_path = Path::new(&self.source_dir);
        let namespace_path = source_path.join(self.namespace[1..].join("/"));

        std::fs::create_dir_all(&namespace_path)?;

        for item in self.file.items.iter() {
            if let Item::Enum(enum_item) = item {
                let enum_ident: IdentName = (&enum_item.ident).into();

                let type_gen_path =
                    namespace_path.join(format!("{}.gen.cpp", enum_ident.to_file_name()));

                // generate impl src
                println!("Generating {}...", type_gen_path.to_str().unwrap());
                {
                    if type_gen_path.exists() {
                        std::fs::remove_file(&type_gen_path)?;
                    }

                    let mut type_gen_file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(type_gen_path)?;
                    enum_impl::write(&mut type_gen_file, enum_item, &self.namespace)?;
                }
            }
        }

        Ok(())
    }
}

pub struct Builder<'a> {
    source_code: &'a str,
    namespace: Option<String>,
    header_dir: Option<String>,
    source_dir: Option<String>,
}

impl<'a> Builder<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            source_code,
            namespace: None,
            header_dir: None,
            source_dir: None,
        }
    }

    pub fn build(self) -> Result<Generator> {
        if self.header_dir.is_none() {
            return Err(Error::ConfigError(
                "header_dir".to_owned(),
                "specify directory to save header files".to_owned(),
            ));
        }

        if self.source_dir.is_none() {
            return Err(Error::ConfigError(
                "source_dir".to_owned(),
                "specify directory to save source files".to_owned(),
            ));
        }

        let namespace = self
            .namespace
            .map(|s| s.split("::").map(|p| p.to_owned()).collect::<Vec<String>>())
            .unwrap_or_default();

        let header_dir = self.header_dir.unwrap();
        let source_dir = self.source_dir.unwrap();

        let file = syn::parse_file(self.source_code)?;

        Ok(Generator {
            file,
            namespace,
            header_dir,
            source_dir,
        })
    }

    pub fn with_namespace(self, namespace: &str) -> Self {
        Self {
            namespace: Some(namespace.to_owned()),
            ..self
        }
    }

    pub fn save_headers_at(self, path: &str) -> Self {
        Self {
            header_dir: Some(path.to_owned()),
            ..self
        }
    }

    pub fn save_sources_at(self, path: &str) -> Self {
        Self {
            source_dir: Some(path.to_owned()),
            ..self
        }
    }
}
