use std::collections::{HashMap, HashSet};

use syn::{Field, Fields, GenericArgument, Path, PathArguments, PathSegment, Type};

use super::{
    error::{Error, Result},
    names::IdentName,
};

pub fn fields_to_cpp_type(ty: &Fields) -> Result<String> {
    Ok(match ty {
        Fields::Unit => "void".to_string(),
        Fields::Named(_) => return Err(Error::UnknownFieldsType(ty.clone())),
        Fields::Unnamed(_) => unnamed_to_cpp_type(ty)?,
    })
}

pub fn fields_to_cpp_dtor(ty: &Fields) -> Result<String> {
    Ok(match ty {
        Fields::Unit => return Err(Error::UnknownFieldsType(ty.clone())),
        Fields::Named(_) => return Err(Error::UnknownFieldsType(ty.clone())),
        Fields::Unnamed(_) => unnamed_to_cpp_dtor(ty)?,
    })
}

fn unnamed_to_cpp_type(unnamed: &Fields) -> Result<String> {
    let fields: Vec<&Field> = unnamed.iter().collect();

    if fields.len() == 0 {
        Ok("void".to_owned())
    } else if fields.len() == 1 {
        Ok(field_to_cpp_type(fields.get(0).unwrap())?)
    } else {
        let field_type_names: Result<Vec<_>> = fields.into_iter().map(field_to_cpp_type).collect();
        Ok(format!("std::tuple<{}>", field_type_names?.join(", ")))
    }
}

fn unnamed_to_cpp_dtor(unnamed: &Fields) -> Result<String> {
    let fields: Vec<&Field> = unnamed.iter().collect();

    if fields.len() == 0 {
        Err(Error::UnknownFieldsType(unnamed.clone()))
    } else if fields.len() == 1 {
        Ok(field_to_cpp_dtor(fields.get(0).unwrap())?)
    } else {
        Ok("~tuple".to_string())
    }
}

fn field_to_cpp_type(field: &Field) -> Result<String> {
    type_to_cpp_type(&field.ty)
}

fn field_to_cpp_dtor(field: &Field) -> Result<String> {
    type_to_cpp_dtor(&field.ty)
}

pub fn type_to_cpp_type(ty: &Type) -> Result<String> {
    match ty {
        Type::Path(type_path) => type_path_to_cpp_type(&type_path.path),
        _ => return Err(Error::UnknownType(ty.clone())),
    }
}

pub fn type_to_cpp_dtor(ty: &Type) -> Result<String> {
    match ty {
        Type::Path(type_path) => type_path_to_cpp_dtor(&type_path.path),
        _ => return Err(Error::UnknownType(ty.clone())),
    }
}

fn type_path_to_cpp_type(path: &Path) -> Result<String> {
    let segments: Result<Vec<_>> = path.segments.iter().map(path_segment_to_cpp_type).collect();
    Ok(segments?.join("::"))
}

fn type_path_to_cpp_dtor(path: &Path) -> Result<String> {
    let last_segment = path.segments.last().unwrap();
    Ok(path_segment_to_cpp_dtor(last_segment)?)
}

fn path_segment_to_cpp_type(segment: &PathSegment) -> Result<String> {
    let template_args = match &segment.arguments {
        PathArguments::None => String::new(),
        PathArguments::AngleBracketed(args) => {
            let args: Result<Vec<_>> = args
                .args
                .iter()
                .filter_map(|g| match g {
                    GenericArgument::Type(t) => Some(t),
                    _ => None,
                })
                .map(type_to_cpp_type)
                .collect();

            format!("<{}>", args?.join(", "))
        }
        _ => return Err(Error::InvalidTypePathSegment(segment.clone())),
    };

    let ident_name = segment.ident.to_string();

    let type_map = HashMap::from([("Vec", "std::vector"), ("String", "std::string")]);

    let cppust_prefixing_types = HashSet::from([
        "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "f32", "f64", "usize", "isize",
    ]);

    let ident_name = if let Some(mapped) = type_map.get(ident_name.as_str()) {
        mapped.to_string()
    } else if cppust_prefixing_types.get(ident_name.as_str()).is_some() {
        format!("::cppust::{}", ident_name)
    } else {
        let ident_name: IdentName = ident_name.as_str().into();
        ident_name.to_class_name()
    };

    Ok(format!("{}{}", ident_name, template_args))
}

fn path_segment_to_cpp_dtor(segment: &PathSegment) -> Result<String> {
    let ident_name = segment.ident.to_string();

    let type_map = HashMap::from([("Vec", "~vector"), ("String", "~string")]);

    let cppust_prefixing_types = HashSet::from([
        "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "f32", "f64", "usize", "isize",
    ]);

    let ident_name = if let Some(mapped) = type_map.get(ident_name.as_str()) {
        mapped.to_string()
    } else if cppust_prefixing_types.get(ident_name.as_str()).is_some() {
        format!("~{}", ident_name)
    } else {
        let ident_name: IdentName = ident_name.as_str().into();
        format!("~{}", ident_name.to_class_name())
    };

    Ok(ident_name)
}
