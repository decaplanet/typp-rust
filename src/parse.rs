use core::panic;
use std::{borrow::BorrowMut, cell::RefCell, error::Error, rc::Rc};

enum SharpKeyword {
    Type(TypeCategory),
    Chunk,
}
enum TypeCategory {
    Chunk,
}
impl std::fmt::Debug for TypeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeCategory::Chunk => write!(f, "chunk"),
        }
    }
}

mod constants {
    pub mod syntax {
        pub const SHARP_PREFIX: &str = "#_";
    }
    pub mod keywords {
        pub const SHARP_KEYWORDS: [&str; 2] = ["type", "chunk"];
        pub const TYPE_CATEGORY: [&str; 1] = ["chunk"];
    }
}

#[allow(dead_code)]
pub fn parse_str(input_str: &str) -> Result<(), Box<dyn Error>> {
    let lines = input_str.split('\n').collect::<Vec<&str>>();
    let mut last_checked_line_index = Rc::new(RefCell::new(0));

    // Iterate each line.
    for (i, line) in lines.as_slice().iter().enumerate() {
        let mut line = *line;
        if i <= *last_checked_line_index.borrow() {
            continue;
        }
        *last_checked_line_index.borrow_mut() = Rc::new(RefCell::new(i));

        // Remove comments from `line`
        if line.starts_with("//") {
            continue;
        }
        if line.contains("//") {
            line = line.split("//").next().unwrap_or_else(|| {
                panic!("Failed to split line {} with comment. Line: {}", i, line)
            })
        }

        if line.starts_with(constants::syntax::SHARP_PREFIX) {
            match read_sharp(i, line) {
                Ok(sharp_keyword) => match sharp_keyword {
                    SharpKeyword::Type(type_category) => {
                        println!("TYPE KEYWORD: {:?}", type_category);

                        println!(
                            "(incoming) last_checked_line_index: {}",
                            *last_checked_line_index.borrow()
                        );

                        match read_type(i, lines.as_slice(), &mut last_checked_line_index) {
                            Ok(()) => {}
                            Err(err) => {
                                panic!(
                                    "Failed to read type type description. Line: {}\nERROR: {}",
                                    i, err
                                );
                            }
                        }
                        println!(
                            "(outgoing) last_checked_line_index: {}",
                            *last_checked_line_index.borrow()
                        );
                    }
                    SharpKeyword::Chunk => {
                        println!("CHUNK KEYWORD");
                        // TODO
                    }
                },
                Err(err) => {
                    panic!("Failed to parse sharp keyword. Line: {}\nERROR: {}", i, err);
                }
            }
        }
    }

    Ok(())
}

fn read_type(
    i: usize,
    lines_slice: &[&str],
    last_checked_line_index: &mut Rc<RefCell<usize>>,
) -> Result<(), Box<dyn Error>> {
    let (at_sign, name) = lines_slice[*last_checked_line_index.borrow() + 1].split_at(1);

    if at_sign != "@" {
        return Err(format!("Expected @ sign. Line: {}", i))?;
    }

    let skip_amount = *last_checked_line_index.borrow() + 2;
    for (j, item) in lines_slice.iter().enumerate().skip(skip_amount) {
        if !(item.starts_with("    ") || item.starts_with("-->")) || item.trim().is_empty() {
            break;
        }
        *last_checked_line_index.borrow_mut() = Rc::new(RefCell::new(j));
        println!("{}, {}", j, item);

        name; // TODO
    }

    Ok(())
}

#[allow(dead_code)]
/// Read sharp keyword.
fn read_sharp(index: usize, line: &str) -> Result<SharpKeyword, Box<dyn Error>> {
    if !line.starts_with(constants::syntax::SHARP_PREFIX) {
        return Err(format!("Unexpected sharp prefix. Line: {}", index))?;
    }

    let temp = line
        .split(constants::syntax::SHARP_PREFIX)
        .skip(1)
        .collect::<Vec<&str>>()
        .join("");

    // Split sharp keyword and argument.
    // e.g. `type(chunk)` -> `type` and `chunk)`
    let sharp_keyword_and_argument = temp.split('(').collect::<Vec<&str>>();

    let sharp_keyword = *sharp_keyword_and_argument
        .first()
        .ok_or(format!("Failed to read sharp keyword. Line: {}", index))?;

    if !constants::keywords::SHARP_KEYWORDS.contains(&sharp_keyword) {
        return Err(format!("Unknown sharp keyword: {}", sharp_keyword))?;
    }

    match sharp_keyword {
        "type" => {
            println!("SHARP KEYWORD: type");

            let type_keyword = sharp_keyword_and_argument
                .last()
                .ok_or(format!("Failed to read sharp keyword. Line: {}", index))?;

            // Syntax validation.
            let is_parenthesis_closed = (*type_keyword).ends_with(')');
            if !is_parenthesis_closed {
                return Err(format!("Expected closing parenthesis. Line: {}", index))?;
            }

            // Keyword check.
            let type_category = type_keyword
                .split(')')
                .next()
                .ok_or(format!("Failed to read sharp keyword. Line: {}", index))?;
            if !constants::keywords::TYPE_CATEGORY.contains(&type_category) {
                panic!("Unknown type keyword: {}", type_category);
            }

            match type_category {
                "chunk" => Ok(SharpKeyword::Type(TypeCategory::Chunk)),
                _ => Err(format!("Unknown type keyword: {}", sharp_keyword))?,
            }
        }
        "chunk" => Ok(SharpKeyword::Chunk),
        _ => Err(format!("Unknown sharp keyword: {}", sharp_keyword))?,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_str() {
        parse_str(
            r#"
#_type(chunk)
@main:
--> document_name: String
    document_author: String

#_type(chunk)
@song_info:
--> name: String
    artist: String[]
    released_year: Int?


#_chunk
@main:
--> document_name: "My Favorite Song"
    document_author: "decaplanet"

#_chunk
favorite_song @song_info:
--> name: "Shelter"
    artist: [
        "Porter Robinson"
        "Madeon"
    ]
    released_year: 2016
"#,
        )
        .unwrap();
    }
}
