use std::io::Write;

use crate::gen::names::IdentName;

use super::error::Result;
use syn::{Fields, ItemEnum};

pub fn write(f: &mut dyn Write, item: &ItemEnum, namespace: &Vec<String>) -> Result<()> {
    let enum_ident_name: IdentName = (&item.ident).into();

    writeln!(
        f,
        "// THIS FILE IS GENERATED AND MANAGED BY cppust-gen, DO NOT CHANGE"
    )?;
    writeln!(
        f,
        "// PLEASE CHANGE {}.cpp INSTEAD",
        enum_ident_name.to_file_name()
    )?;
    writeln!(f)?;

    writeln!(
        f,
        "#include \"{}.hpp\"",
        namespace
            .iter()
            .chain(vec![enum_ident_name.to_file_name()].iter())
            .map(|x| x.clone())
            .collect::<Vec<_>>()
            .join("/")
    )?;
    writeln!(f)?;

    if !namespace.is_empty() {
        writeln!(
            f,
            "{}",
            namespace
                .iter()
                .map(|ns| format!("namespace {} {{", ns))
                .collect::<Vec<String>>()
                .join(" ")
        )?;
        writeln!(f)?;
    }

    write_ctors(f, item)?;
    writeln!(f)?;
    write_dtors(f, item)?;
    writeln!(f)?;
    write_operators(f, item)?;
    writeln!(f)?;
    write_enum_ctors(f, item)?;
    write_accessors(f, item)?;
    write_private_methods(f, item)?;

    if !namespace.is_empty() {
        writeln!(f)?;
        writeln!(
            f,
            "{}",
            namespace
                .iter()
                .map(|_| "}".to_owned())
                .collect::<Vec<String>>()
                .join(" ")
        )?;
    }

    Ok(())
}

fn write_ctors(f: &mut dyn Write, item: &ItemEnum) -> Result<()> {
    let enum_ident_name: IdentName = (&item.ident).into();

    writeln!(f, "// public constructors")?;
    writeln!(
        f,
        "{0}::{0}(const {0}& rhs): {0}(rhs.tag_, rhs.union_) {{ }} // copy constructor",
        enum_ident_name.to_class_name()
    )?;
    writeln!(
        f,
        "{0}::{0}({0}&& rhs): {0}(rhs.tag_, std::move(rhs.union_)) {{ }} // move constructor",
        enum_ident_name.to_class_name()
    )?;

    writeln!(f)?;
    writeln!(f, "// private constructors")?;
    writeln!(
        f,
        "{0}::{0}(_Tag tag): tag_(tag) {{ }}",
        enum_ident_name.to_class_name()
    )?;
    writeln!(f)?;
    writeln!(
        f,
        "{0}::{0}(_Tag tag, const _Union& union_val): tag_(tag) {{",
        enum_ident_name.to_class_name()
    )?;
    writeln!(f, "    tagged_init_(tag, union_val);")?;
    writeln!(f, "}}")?;
    writeln!(f)?;
    writeln!(
        f,
        "{0}::{0}(_Tag tag, _Union&& union_val): tag_(tag) {{",
        enum_ident_name.to_class_name()
    )?;
    writeln!(f, "    tagged_init_(tag, std::move(union_val));")?;
    writeln!(f, "}}")?;

    writeln!(f)?;

    Ok(())
}

fn write_dtors(f: &mut dyn Write, item: &ItemEnum) -> Result<()> {
    let enum_ident_name: IdentName = (&item.ident).into();

    writeln!(f, "// destructor")?;
    writeln!(f, "{0}::~{0}() {{", enum_ident_name.to_class_name())?;
    writeln!(f, "    deinit_union_();")?;
    writeln!(f, "}}")?;
    Ok(())
}

fn write_operators(f: &mut dyn Write, item: &ItemEnum) -> Result<()> {
    let enum_ident_name: IdentName = (&item.ident).into();

    writeln!(f, "// operators")?;
    writeln!(
        f,
        "{0}& {0}::operator=(const {0}& rhs) {{ // assign",
        enum_ident_name.to_class_name()
    )?;
    writeln!(f, "    if (this == &rhs) {{ return *this; }}")?;
    writeln!(f)?;
    writeln!(f, "    if (tag_ == rhs.tag_) {{")?;
    writeln!(f, "        switch (tag_) {{")?;
    for variant in item.variants.iter() {
        let variant_ident_name: IdentName = (&variant.ident).into();

        if let Fields::Unit = &variant.fields {
            continue;
        }

        writeln!(
            f,
            "        case _Tag::{}:",
            variant_ident_name.to_enum_variant_name()
        )?;
        writeln!(
            f,
            "            union_.{0}_val = rhs.union_.{0}_val;",
            variant_ident_name.to_public_member_name()
        )?;
        writeln!(f, "            break;")?;
    }
    writeln!(f, "        default:")?;
    writeln!(f, "            break;")?;
    writeln!(f, "        }}")?;
    writeln!(f, "    }} else {{")?;
    writeln!(f, "        deinit_union_();")?;
    writeln!(f, "        tag_ = rhs.tag_;")?;
    writeln!(f, "        tagged_init_(rhs.tag_, rhs.union_);")?;
    writeln!(f, "    }}")?;
    writeln!(f, "    return *this;")?;
    writeln!(f, "}}")?;
    writeln!(f)?;

    writeln!(
        f,
        "{0}& {0}::operator=({0}&& rhs) {{ // move",
        enum_ident_name.to_class_name()
    )?;
    writeln!(f, "    if (this == &rhs) {{ return *this; }}")?;
    writeln!(f)?;
    writeln!(f, "    if (tag_ == rhs.tag_) {{")?;
    writeln!(f, "        switch (tag_) {{")?;
    for variant in item.variants.iter() {
        let variant_ident_name: IdentName = (&variant.ident).into();

        if let Fields::Unit = &variant.fields {
            continue;
        }

        writeln!(
            f,
            "        case _Tag::{}:",
            variant_ident_name.to_enum_variant_name()
        )?;
        writeln!(
            f,
            "            union_.{0}_val = std::move(rhs.union_.{0}_val);",
            variant_ident_name.to_public_member_name()
        )?;
        writeln!(f, "            break;")?;
    }
    writeln!(f, "        default:")?;
    writeln!(f, "            break;")?;
    writeln!(f, "        }}")?;
    writeln!(f, "    }} else {{")?;
    writeln!(f, "        deinit_union_();")?;
    writeln!(f, "        tag_ = rhs.tag_;")?;
    writeln!(f, "        tagged_init_(rhs.tag_, std::move(rhs.union_));")?;
    writeln!(f, "    }}")?;
    writeln!(f, "    return *this;")?;
    writeln!(f, "}}")?;
    writeln!(f)?;
    writeln!(
        f,
        "bool {0}::operator==(const {0}& rhs) const {{ // equal",
        enum_ident_name.to_class_name()
    )?;
    writeln!(f, "    if (this == &rhs) {{ return true; }}")?;
    writeln!(f, "    if (tag_ != rhs.tag_) {{ return false; }}")?;
    writeln!(f, "    switch (tag_) {{")?;
    for variant in item.variants.iter() {
        let variant_ident_name: IdentName = (&variant.ident).into();

        if let Fields::Unit = &variant.fields {
            continue;
        }

        writeln!(
            f,
            "    case _Tag::{}:",
            variant_ident_name.to_enum_variant_name()
        )?;
        writeln!(
            f,
            "        return union_.{0}_val == rhs.union_.{0}_val;",
            variant_ident_name.to_public_member_name()
        )?;
    }
    writeln!(f, "    default:")?;
    writeln!(f, "        break;")?;
    writeln!(f, "    }}")?;
    writeln!(f, "    return true;")?;
    writeln!(f, "}}")?;
    writeln!(f)?;
    writeln!(
        f,
        "bool {0}::operator!=(const {0}& rhs) const {{ // not equal",
        enum_ident_name.to_class_name()
    )?;
    writeln!(f, "    return !(*this == rhs);")?;
    writeln!(f, "}}")?;

    Ok(())
}

fn write_enum_ctors(f: &mut dyn Write, item: &ItemEnum) -> Result<()> {
    let enum_ident_name: IdentName = (&item.ident).into();

    writeln!(f, "// enum constructors")?;
    for variant in item.variants.iter() {
        let variant_ident_name: IdentName = (&variant.ident).into();

        if let Fields::Unit = &variant.fields {
            writeln!(
                f,
                "{0} {0}::{1}() {{",
                enum_ident_name.to_class_name(),
                variant_ident_name.to_class_name()
            )?;
            writeln!(
                f,
                "    return {0}(_Tag::{1});",
                enum_ident_name.to_class_name(),
                variant_ident_name.to_class_name()
            )?;
            writeln!(f, "}}")?;
        } else {
            write!(
                f,
                "{0} {0}::{1}(",
                enum_ident_name.to_class_name(),
                variant_ident_name.to_class_name(),
            )?;
            let mut cnt = 0;
            for (i, field) in variant.fields.iter().enumerate() {
                if i != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "const {}& v{}", super::types::type_to_cpp_type(&field.ty)?, i)?;
                cnt = i + 1;
            }
            writeln!(f, ") {{")?;
            writeln!(
                f,
                "    {0} output(_Tag::{1});",
                enum_ident_name.to_class_name(),
                variant_ident_name.to_class_name()
            )?;
            if cnt == 1 {
                writeln!(
                    f,
                    "    new (&output.union_.{0}_val) {1}(v0);",
                    variant_ident_name.to_public_member_name(),
                    super::types::fields_to_cpp_type(&variant.fields)?
                )?;
            } else {
                write!(
                    f,
                    "    new (&output.union_.{0}_val) {1}(",
                    variant_ident_name.to_public_member_name(),
                    super::types::fields_to_cpp_type(&variant.fields)?
                )?;
                for i in 0..cnt {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "v{}", i)?;
                }
                writeln!(f, ");")?;
            }
            writeln!(f, "    return output;")?;
            writeln!(f, "}}")?;
        }
        writeln!(f)?;
    }
    Ok(())
}

fn write_accessors(f: &mut dyn Write, item: &ItemEnum) -> Result<()> {
    let enum_ident_name: IdentName = (&item.ident).into();

    writeln!(f, "// accessors")?;

    for variant in item.variants.iter() {
        let variant_ident_name: IdentName = (&variant.ident).into();

        // is
        writeln!(f, "bool {}::is_{}() const {{", enum_ident_name.to_class_name(), variant_ident_name.to_public_member_name())?;
        writeln!(f, "    return tag_ == _Tag::{};", variant_ident_name.to_enum_variant_name())?;
        writeln!(f, "}}")?;
        writeln!(f)?;

        if let Fields::Unit = &variant.fields {
            continue;
        }

        let value_type = super::types::fields_to_cpp_type(&variant.fields)?;

        // ref_uncheck
        writeln!(
            f,
            "const {}& {}::{}_ref_uncheck() const {{",
            &value_type,
            enum_ident_name.to_class_name(),
            variant_ident_name.to_public_member_name(),
        )?;
        writeln!(f, "    assert(tag_ == _Tag::{});", variant_ident_name.to_enum_variant_name())?;
        writeln!(f, "    return union_.{}_val;", variant_ident_name.to_public_member_name())?;
        writeln!(f, "}}")?;
        writeln!(f)?;

        writeln!(
            f,
            "{}& {}::{}_ref_uncheck() {{",
            &value_type,
            enum_ident_name.to_class_name(),
            variant_ident_name.to_public_member_name(),
        )?;
        writeln!(f, "    assert(tag_ == _Tag::{});", variant_ident_name.to_enum_variant_name())?;
        writeln!(f, "    return union_.{}_val;", variant_ident_name.to_public_member_name())?;
        writeln!(f, "}}")?;
        writeln!(f)?;

        // ref
        writeln!(
            f,
            "const {}& {}::{}_ref() const {{",
            &value_type,
            enum_ident_name.to_class_name(),
            variant_ident_name.to_public_member_name(),
        )?;
        writeln!(f, "    if (tag_ != _Tag::{}) {{", variant_ident_name.to_enum_variant_name())?;
        writeln!(f, "        throw std::runtime_error(\"requires {}\");", variant_ident_name.to_enum_variant_name())?;
        writeln!(f, "    }}")?;
        writeln!(f, "    return union_.{}_val;", variant_ident_name.to_public_member_name())?;
        writeln!(f, "}}")?;
        writeln!(f)?;

        writeln!(
            f,
            "{}& {}::{}_ref() {{",
            &value_type,
            enum_ident_name.to_class_name(),
            variant_ident_name.to_public_member_name(),
        )?;
        writeln!(f, "    if (tag_ != _Tag::{}) {{", variant_ident_name.to_enum_variant_name())?;
        writeln!(f, "        throw std::runtime_error(\"requires {}\");", variant_ident_name.to_enum_variant_name())?;
        writeln!(f, "    }}")?;
        writeln!(f, "    return union_.{}_val;", variant_ident_name.to_public_member_name())?;
        writeln!(f, "}}")?;
        writeln!(f)?;

        // ptr
        writeln!(
            f,
            "const {}* {}::{}_ptr() const {{",
            &value_type,
            enum_ident_name.to_class_name(),
            variant_ident_name.to_public_member_name(),
        )?;
        writeln!(f, "    if (tag_ != _Tag::{}) {{", variant_ident_name.to_enum_variant_name())?;
        writeln!(f, "        return nullptr;")?;
        writeln!(f, "    }}")?;
        writeln!(f, "    return &union_.{}_val;", variant_ident_name.to_public_member_name())?;
        writeln!(f, "}}")?;
        writeln!(f)?;

        writeln!(
            f,
            "{}* {}::{}_ptr() {{",
            &value_type,
            enum_ident_name.to_class_name(),
            variant_ident_name.to_public_member_name(),
        )?;
        writeln!(f, "    if (tag_ != _Tag::{}) {{", variant_ident_name.to_enum_variant_name())?;
        writeln!(f, "        return nullptr;")?;
        writeln!(f, "    }}")?;
        writeln!(f, "    return &union_.{}_val;", variant_ident_name.to_public_member_name())?;
        writeln!(f, "}}")?;
        writeln!(f)?;
    }

    Ok(())
}

fn write_private_methods(f: &mut dyn Write, item: &ItemEnum) -> Result<()> {
    let enum_ident_name: IdentName = (&item.ident).into();

    writeln!(f, "// private methods")?;

    // tagged_init_
    writeln!(f, "void {}::tagged_init_(_Tag tag, const _Union& union_val) {{", enum_ident_name.to_class_name())?;
    writeln!(f, "    switch (tag) {{")?;
    for variant in item.variants.iter() {
        if let Fields::Unit = &variant.fields {
            continue;
        }

        let value_type = super::types::fields_to_cpp_type(&variant.fields)?;
        let variant_ident_name: IdentName = (&variant.ident).into();

        writeln!(f, "    case _Tag::{}:", variant_ident_name.to_enum_variant_name())?;
        writeln!(
            f,
            "        new (&union_.{0}_val) {1}(union_val.{0}_val);",
            variant_ident_name.to_public_member_name(),
            &value_type
        )?;
        writeln!(f, "        break;")?;
    }
    writeln!(f, "    default:")?;
    writeln!(f, "        break;")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}")?;
    writeln!(f)?;

    // tagged_init_ (move)
    writeln!(f, "void {}::tagged_init_(_Tag tag, _Union&& union_val) {{", enum_ident_name.to_class_name())?;
    writeln!(f, "    switch (tag) {{")?;
    for variant in item.variants.iter() {
        if let Fields::Unit = &variant.fields {
            continue;
        }

        let value_type = super::types::fields_to_cpp_type(&variant.fields)?;
        let variant_ident_name: IdentName = (&variant.ident).into();

        writeln!(f, "    case _Tag::{}:", variant_ident_name.to_enum_variant_name())?;
        writeln!(
            f,
            "        new (&union_.{0}_val) {1}(std::move(union_val.{0}_val));",
            variant_ident_name.to_public_member_name(),
            &value_type
        )?;
        writeln!(f, "        break;")?;
    }
    writeln!(f, "    default:")?;
    writeln!(f, "        break;")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}")?;
    writeln!(f)?;

    // deinit_union_
    writeln!(f, "void {}::deinit_union_() {{", enum_ident_name.to_class_name())?;
    writeln!(f, "    switch (tag_) {{")?;
    for variant in item.variants.iter() {
        if let Fields::Unit = &variant.fields {
            continue;
        }

        let variant_ident_name: IdentName = (&variant.ident).into();

        writeln!(f, "    case _Tag::{}:", variant_ident_name.to_enum_variant_name())?;
        writeln!(
            f,
            "        union_.{0}_val.{1}();",
            variant_ident_name.to_public_member_name(),
            super::types::fields_to_cpp_dtor(&variant.fields)?,
        )?;
        writeln!(f, "        break;")?;
    }
    writeln!(f, "    default:")?;
    writeln!(f, "        break;")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}")?;
    writeln!(f)?;

    Ok(())
}
