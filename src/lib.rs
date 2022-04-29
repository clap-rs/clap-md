extern crate clap;
extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;

use clap::Command;
use pulldown_cmark::{Event, HeadingLevel, Tag};
use pulldown_cmark_to_cmark::cmark;

struct Document<'a>(Vec<Event<'a>>);

impl<'a> Document<'a> {
    fn header(&mut self, text: String, level: i32) {
        let level = match level {
            1 => HeadingLevel::H1,
            2 => HeadingLevel::H2,
            3 => HeadingLevel::H3,
            4 => HeadingLevel::H4,
            5 => HeadingLevel::H5,
            6 => HeadingLevel::H6,
            _ => panic!("nope"),
        };
        self.0
            .push(Event::Start(Tag::Heading(level, None, Vec::new())));
        self.0.push(Event::Text(text.into()));
        self.0
            .push(Event::End(Tag::Heading(level, None, Vec::new())));
    }

    fn paragraph(&mut self, text: String) {
        self.0.push(Event::Start(Tag::Paragraph));
        self.0.push(Event::Text(text.into()));
        self.0.push(Event::End(Tag::Paragraph));
    }
}

fn recursive(doc: &mut Document, app: &Command, level: i32, skip_header: bool) {
    if !skip_header {
        doc.header(app.get_name().into(), level);
    }

    if let Some(about) = app.get_about() {
        doc.paragraph(about.into());
    }
    if let Some(author) = app.get_author() {
        doc.paragraph(format!("Author: {}", author));
    }
    if let Some(version) = app.get_version() {
        doc.paragraph(format!("Version: {}", version));
    }

    if app.get_arguments().any(|_| true) {
        doc.paragraph("Arguments:".into());
        doc.0.push(Event::Start(Tag::List(None)));

        for arg in app.get_arguments() {
            doc.0.push(Event::Start(Tag::Item));
            doc.0.push(Event::Start(Tag::Paragraph));

            let mut def = String::new();
            if let Some(short) = arg.get_short() {
                def.push_str("-");
                def.push(short);
            }
            if let Some(long) = arg.get_long() {
                if arg.get_short().is_some() {
                    def.push_str("/");
                }
                def.push_str("--");
                def.push_str(long);
            }

            if arg.is_takes_value_set() {
                def.push_str("=<");
                def.push_str(arg.get_id());
                def.push_str(">");
            }

            doc.0.push(Event::Code(def.into()));

            let mut text = String::new();
            if let Some(help) = arg.get_help() {
                if arg.get_short().is_some() || arg.get_long().is_some() {
                    text.push_str(": ");
                }
                text.push_str(help);
            }
            
            doc.0.push(Event::Text(text.into()));

            doc.0.push(Event::End(Tag::Paragraph));
            doc.0.push(Event::End(Tag::Item));
        }

        doc.0.push(Event::End(Tag::List(None)));
    }

    if app.get_subcommands().any(|_| true) {
        doc.header("Subcommands".into(), level + 1);

        for cmd in app.get_subcommands() {
            recursive(doc, cmd, level + 2, false);
        }
    }
}

/// Convert a clap App to markdown documentation
///
/// # Parameters
///
/// - `app`: A reference to a clap application definition
/// - `level`: The level for first markdown headline. If you for example want to
///     render this beneath a `## Usage` headline in your readme, you'd want to
///     set `level` to `2`.
pub fn app_to_md<'a, 'b>(app: &Command, level: i32) -> Result<String, Box<dyn std::error::Error>> {
    let mut document = Document(Vec::new());
    recursive(&mut document, app, level, level > 1);
    let mut result = String::new();
    cmark(document.0.iter(), &mut result)?;
    Ok(result)
}