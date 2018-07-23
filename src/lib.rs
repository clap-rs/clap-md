extern crate clap;
extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;

use clap::{App, ArgSettings};
use pulldown_cmark::{Event, Tag};
use pulldown_cmark_to_cmark::fmt::cmark;

struct Document<'a>(Vec<Event<'a>>);

impl<'a> Document<'a> {
    fn header(&mut self, text: String, level: i32) {
        self.0.push(Event::Start(Tag::Header(level)));
        self.0.push(Event::Text(text.into()));
        self.0.push(Event::End(Tag::Header(level)));
    }

    fn paragraph(&mut self, text: String) {
        self.0.push(Event::Start(Tag::Paragraph));
        self.0.push(Event::Text(text.into()));
        self.0.push(Event::End(Tag::Paragraph));
    }
}

fn recursive(doc: &mut Document, app: &App, level: i32, skip_header: bool) {
    if !skip_header {
        doc.header(app.name.clone(), level);
    }

    if let Some(about) = app.about {
        doc.paragraph(about.into());
    }
    if let Some(author) = app.author {
        doc.paragraph(format!("Author: {}", author));
    }
    if let Some(version) = app.version_short {
        let msg = if let Some(msg) = app.version_message {
            format!(" ({})", msg)
        } else {
            "".into()
        };
        doc.paragraph(format!("Version: {}{}", version, msg));
    }

    if !app.args.is_empty() {
        doc.paragraph("Arguments:".into());
        doc.0.push(Event::Start(Tag::List(None)));

        for arg in &app.args {
            doc.0.push(Event::Start(Tag::Item));
            doc.0.push(Event::Start(Tag::Paragraph));

            doc.0.push(Event::Start(Tag::Code));

            let mut def = String::new();
            if let Some(short) = arg.short {
                def.push_str("-");
                def.push(short);
            }
            if let Some(long) = arg.long {
                if arg.short.is_some() {
                    def.push_str("/");
                }
                def.push_str("--");
                def.push_str(long);
            }

            if arg.is_set(ArgSettings::TakesValue) {
                def.push_str("=<");
                def.push_str(arg.name);
                def.push_str(">");
            }

            doc.0.push(Event::Text(def.into()));
            doc.0.push(Event::End(Tag::Code));

            let mut text = String::new();
            if let Some(help) = arg.help {
                if arg.short.is_some() || arg.long.is_some() {
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

    if !app.subcommands.is_empty() {
        doc.header("Subcommands".into(), level + 1);

        for cmd in &app.subcommands {
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
pub fn app_to_md<'a, 'b>(
    app: &App<'a, 'b>,
    level: i32,
) -> Result<String, Box<::std::error::Error>> {
    let mut document = Document(Vec::new());
    recursive(&mut document, app, level, level > 1);
    let mut result = String::new();
    cmark(document.0.iter(), &mut result, None)?;
    Ok(result)
}
