use syn::Ident;

#[derive(Clone, Debug, PartialEq)]
pub struct IdentName(Vec<IdentPart>);

impl From<&str> for IdentName {
    fn from(f: &str) -> Self {
        Self(parse_ident_name(f))
    }
}

impl From<&Ident> for IdentName {
    fn from(ident: &Ident) -> Self {
        ident.to_string().as_str().into()
    }
}

impl IdentName {
    pub fn to_class_name(&self) -> String {
        self.to_pascal_case()
    }

    pub fn to_file_name(&self) -> String {
        self.to_snake()
    }

    // pub fn to_template_arg(&self) -> String {
    //     self.to_pascal_case()
    // }

    // pub fn to_enum_name(&self) -> String {
    //     self.to_pascal_case()
    // }

    pub fn to_enum_variant_name(&self) -> String {
        self.to_pascal_case()
    }

    // pub fn to_const_name(&self) -> String {
    //     format!("k{}", self.to_pascal_case())
    // }

    pub fn to_public_member_name(&self) -> String {
        self.to_snake()
    }

    // pub fn to_protected_member_name(&self) -> String {
    //     self.to_snake()
    // }

    // pub fn to_private_member_name(&self) -> String {
    //     format!("{}_", self.to_snake())
    // }

    // pub fn to_variable_name(&self) -> String {
    //     self.to_camel_case()
    // }

    fn to_snake(&self) -> String {
        self.0
            .iter()
            .map(|s| s.to_all_lower_case())
            .collect::<Vec<String>>()
            .join("_")
    }

    // fn to_camel_case(&self) -> String {
    //     self.0
    //         .iter()
    //         .enumerate()
    //         .map(|(idx, s)| {
    //             if idx == 0 {
    //                 s.to_all_lower_case()
    //             } else {
    //                 s.to_capital_first()
    //             }
    //         })
    //         .collect::<Vec<String>>()
    //         .join("")
    // }

    fn to_pascal_case(&self) -> String {
        self.0
            .iter()
            .map(|s| s.to_capital_first())
            .collect::<Vec<String>>()
            .join("")
    }
}

#[derive(Clone, Debug, PartialEq)]
enum IdentPart {
    CapitalFirst(String),
    AllUpperCase(String),
    AllLowerCase(String),
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

impl IdentPart {
    pub fn to_capital_first(&self) -> String {
        match self {
            IdentPart::CapitalFirst(s) => s.clone(),
            IdentPart::AllUpperCase(s) => s.clone(),
            IdentPart::AllLowerCase(s) => uppercase_first_letter(s),
        }
    }

    // pub fn to_all_upper_case(&self) -> String {
    //     match self {
    //         IdentPart::CapitalFirst(s) => s.to_uppercase(),
    //         IdentPart::AllUpperCase(s) => s.clone(),
    //         IdentPart::AllLowerCase(s) => s.to_uppercase(),
    //     }
    // }

    pub fn to_all_lower_case(&self) -> String {
        match self {
            IdentPart::CapitalFirst(s) => s.to_lowercase(),
            IdentPart::AllUpperCase(s) => s.to_lowercase(),
            IdentPart::AllLowerCase(s) => s.clone(),
        }
    }
}

fn segment_ident_name(ident: &str) -> Vec<String> {
    let mut parsing_part = String::new();

    enum ParsingStatus {
        WaitingNextSegment,
        FirstUpperCase,
        ReadingAllUpperCase,
        ReadingAllLowerCase,
        ReadingCapitalFirst,
    }

    let mut segments = Vec::new();
    let mut status = ParsingStatus::WaitingNextSegment;

    for char in ident.chars() {
        status = match status {
            ParsingStatus::WaitingNextSegment => {
                parsing_part.push(char);
                if char.is_uppercase() {
                    ParsingStatus::FirstUpperCase
                } else {
                    ParsingStatus::ReadingAllLowerCase
                }
            }
            ParsingStatus::FirstUpperCase => {
                if char == '_' {
                    segments.push(parsing_part.clone());
                    parsing_part.clear();
                    ParsingStatus::WaitingNextSegment
                } else if char.is_uppercase() {
                    parsing_part.push(char);
                    ParsingStatus::ReadingAllUpperCase
                } else {
                    parsing_part.push(char);
                    ParsingStatus::ReadingCapitalFirst
                }
            }
            ParsingStatus::ReadingAllUpperCase => {
                if char == '_' {
                    segments.push(parsing_part.clone());
                    parsing_part.clear();
                    ParsingStatus::WaitingNextSegment
                } else if !char.is_lowercase() {
                    parsing_part.push(char);
                    ParsingStatus::ReadingAllUpperCase
                } else {
                    let (last_char_idx, last_char) = parsing_part.char_indices().last().unwrap();
                    parsing_part.remove(last_char_idx);
                    segments.push(parsing_part.clone());
                    parsing_part.clear();
                    parsing_part.push(last_char);
                    parsing_part.push(char);
                    ParsingStatus::FirstUpperCase
                }
            }
            ParsingStatus::ReadingAllLowerCase => {
                if char == '_' {
                    segments.push(parsing_part.clone());
                    parsing_part.clear();
                    ParsingStatus::WaitingNextSegment
                } else if char.is_uppercase() {
                    segments.push(parsing_part.clone());
                    parsing_part.clear();
                    parsing_part.push(char);
                    ParsingStatus::FirstUpperCase
                } else {
                    parsing_part.push(char);
                    ParsingStatus::ReadingAllLowerCase
                }
            }
            ParsingStatus::ReadingCapitalFirst => {
                if char == '_' {
                    segments.push(parsing_part.clone());
                    parsing_part.clear();
                    ParsingStatus::WaitingNextSegment
                } else if char.is_uppercase() {
                    segments.push(parsing_part.clone());
                    parsing_part.clear();
                    parsing_part.push(char);
                    ParsingStatus::FirstUpperCase
                } else {
                    parsing_part.push(char);
                    ParsingStatus::ReadingAllLowerCase
                }
            }
        };
    }

    if !parsing_part.is_empty() {
        segments.push(parsing_part);
    }

    segments
}

fn parse_ident_part(s: String) -> IdentPart {
    let mut has_upper_case = false;
    let mut has_lower_case = false;

    for char in s.chars() {
        if char.is_uppercase() {
            has_upper_case = true;
        } else if char.is_lowercase() {
            has_lower_case = true;
        }
    }

    if has_upper_case && !has_lower_case {
        IdentPart::AllUpperCase(s)
    } else if has_upper_case {
        IdentPart::CapitalFirst(s)
    } else {
        IdentPart::AllLowerCase(s)
    }
}

fn parse_ident_name(ident: &str) -> Vec<IdentPart> {
    let segments = segment_ident_name(ident);
    segments.into_iter().map(parse_ident_part).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_segment_ident_name() {
        assert_eq!(segment_ident_name("lower"), vec!["lower".to_owned()]);
        assert_eq!(segment_ident_name("UPPER"), vec!["UPPER".to_owned()]);
        assert_eq!(segment_ident_name("Capital"), vec!["Capital".to_owned()]);
        assert_eq!(
            segment_ident_name("under_score"),
            vec!["under".to_owned(), "score".to_owned()]
        );
        assert_eq!(
            segment_ident_name("USBDriver"),
            vec!["USB".to_owned(), "Driver".to_owned()]
        );
        assert_eq!(
            segment_ident_name("USB_Driver"),
            vec!["USB".to_owned(), "Driver".to_owned()]
        );
        assert_eq!(
            segment_ident_name("camelCase"),
            vec!["camel".to_owned(), "Case".to_owned()]
        );
        assert_eq!(
            segment_ident_name("camelCase123"),
            vec!["camel".to_owned(), "Case123".to_owned()]
        );
        assert_eq!(
            segment_ident_name("USB2Driver"),
            vec!["USB2".to_owned(), "Driver".to_owned()]
        );
        assert_eq!(
            segment_ident_name("Usb2Driver"),
            vec!["Usb2".to_owned(), "Driver".to_owned()]
        );
        assert_eq!(
            segment_ident_name("usb2_driver"),
            vec!["usb2".to_owned(), "driver".to_owned()]
        );
    }

    #[test]
    fn test_parse_ident_part() {
        assert_eq!(
            parse_ident_part("lower".to_owned()),
            IdentPart::AllLowerCase("lower".to_owned())
        );
        assert_eq!(
            parse_ident_part("UPPER".to_owned()),
            IdentPart::AllUpperCase("UPPER".to_owned())
        );
        assert_eq!(
            parse_ident_part("Capital".to_owned()),
            IdentPart::CapitalFirst("Capital".to_owned())
        );
        assert_eq!(
            parse_ident_part("Case123".to_owned()),
            IdentPart::CapitalFirst("Case123".to_owned())
        );
        assert_eq!(
            parse_ident_part("USB2".to_owned()),
            IdentPart::AllUpperCase("USB2".to_owned())
        );
        assert_eq!(
            parse_ident_part("Usb2".to_owned()),
            IdentPart::CapitalFirst("Usb2".to_owned())
        );
        assert_eq!(
            parse_ident_part("usb2".to_owned()),
            IdentPart::AllLowerCase("usb2".to_owned())
        );
    }
}
