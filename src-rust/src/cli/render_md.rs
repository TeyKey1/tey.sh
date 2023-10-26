//! Render markdown content to the terminal

use colored::*;
use markdown::mdast::{Emphasis, Heading, Link, List, Node, Paragraph, Strong};
use markdown::ParseOptions;

pub fn render_md(md: &str) -> String {
    let ast = markdown::to_mdast(md, &ParseOptions::default())
        .expect("Failed to parse the provided markdown string");

    let mut content = String::new();

    if let Some(children) = ast.children() {
        for child in children {
            match child {
                Node::List(list) => render_list(list, &mut content),
                Node::Heading(heading) => render_heading(heading, &mut content),
                Node::Paragraph(paragraph) => render_paragraph(paragraph, &mut content),
                other => panic!(
                    "Top level rendering of markdown node not supported: {:?} ",
                    other
                ),
            }
        }
    }

    /*traverse_md_tree(&ast, &|node| {
        log::debug!("{:?}", node);
    });*/

    content
}

fn render_heading(heading: &Heading, content: &mut String) {
    let mut heading_content = String::new();

    match heading.depth {
        1 => heading_content.push_str(&format!("\n\n{} ", "#".yellow().bold())),
        2 => heading_content.push_str(&format!("\n{} ", "##".magenta().bold())),
        3 => heading_content.push_str(&format!("\n{} ", "###".bold())),
        _ => panic!("Markdown headings below H3 are not supported"),
    }

    render_text(&heading.children, &mut heading_content);

    content.push_str(&format!("{}\n", heading_content.bold()));
}

fn render_paragraph(paragraph: &Paragraph, content: &mut String) {
    render_text(&paragraph.children, content);

    content.push_str("\n\n");
}

fn render_list(list: &List, content: &mut String) {
    let mut entry_nbr: usize = 1;

    for child in list.children.iter() {
        match child {
            Node::ListItem(list_item) => {
                let mut list_text = String::new();

                list_text.push_str(&format!("{}. ", entry_nbr));

                render_text(&list_item.children, &mut list_text);

                content.push_str(&list_text);

                entry_nbr += 1;
            }
            other => panic!(
                "Encountered unsupported markdown in list child: {:?}",
                other
            ),
        }
    }

    content.push_str("\n");
}

fn render_text(nodes: &Vec<Node>, content: &mut String) {
    for child in nodes {
        match child {
            Node::Paragraph(paragraph) => render_paragraph(paragraph, content),
            Node::Emphasis(emphasis) => render_emphasis(emphasis, content),
            Node::Link(link) => render_link(link, content),
            Node::Strong(strong) => render_strong(strong, content),
            Node::Text(text) => content.push_str(&text.value),
            other => panic!(
                "Encountered unsupported markdown in text child: {:?}",
                other
            ),
        }
    }
}

fn render_strong(strong: &Strong, content: &mut String) {
    let mut strong_text = String::new();

    render_text(&strong.children, &mut strong_text);

    let strong = strong_text.bold();

    content.push_str(&strong.to_string());
}

fn render_emphasis(emphasis: &Emphasis, content: &mut String) {
    let mut emphasis_text = String::new();

    render_text(&emphasis.children, &mut emphasis_text);

    let emphasis = emphasis_text.italic().dimmed();

    content.push_str(&emphasis.to_string());
}

fn render_link(link: &Link, content: &mut String) {
    let mut link_text = String::new();

    render_text(&link.children, &mut link_text);

    let link = format!("[{}]({}) ", link_text, link.url.underline()).green();

    content.push_str(&link.to_string());
}

/// Traverses the provided md ast tree starting from the provided node and executes the provided closure on each encountered node in a depth-first approach.
fn traverse_md_tree<'a, F: Fn(&'a Node)>(node: &'a Node, f: &F) {
    f(node);

    if let Some(children) = node.children() {
        for child in children {
            traverse_md_tree(child, f);
        }
    }
}
