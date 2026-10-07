//! A `<td>` restyled `display: block` must still lay out its contents.
//!
//! CSS 2.2 §17.2.1: a child of a table row that is not a table-cell is
//! wrapped in an anonymous table-cell box. Browsers render the content.
//!
//! Regression test from HTML email: the widely copied "responsive HTML email
//! template" (leemunroe/responsive-html-email-template) restyles its
//! `.container` cell with `td.container { display: block; max-width: 580px }`.
//! Without the anonymous cell, the cell and the entire message body inside it
//! were dropped and the email rendered blank.

use blitz_dom::{DocumentConfig, FontContext};
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::shell::{ColorScheme, Viewport};
use std::sync::Arc;

fn layout_doc(html: &str) -> HtmlDocument {
    let mut doc = HtmlDocument::from_html(
        html,
        DocumentConfig {
            viewport: Some(Viewport::new(800, 600, 1.0, ColorScheme::Light)),
            html_parser_provider: Some(Arc::new(HtmlProvider) as _),
            font_ctx: Some(FontContext::new()),
            ..Default::default()
        },
    );
    doc.resolve(0.0);
    doc
}

#[test]
fn block_td_contents_are_laid_out() {
    let doc = layout_doc(
        r#"<html><body style="margin:0">
            <table style="width:600px"><tr>
                <td style="width:100px">left</td>
                <td id="cell" style="display:block; max-width:400px">
                    <div id="content" style="height:50px">body</div>
                </td>
            </tr></table>
        </body></html>"#,
    );
    let cell = doc.query_selector("#cell").unwrap().unwrap();
    let content = doc.query_selector("#content").unwrap().unwrap();
    let cell_size = doc.get_node(cell).unwrap().final_layout().size;
    let content_size = doc.get_node(content).unwrap().final_layout().size;
    assert!(
        cell_size.width > 0.0 && cell_size.height >= 50.0,
        "cell {cell_size:?}"
    );
    assert_eq!(content_size.height, 50.0);
    assert!(content_size.width > 0.0, "content {content_size:?}");
}

#[test]
fn block_td_as_only_cell() {
    // The template's shape: the block cell is the row's only cell.
    let doc = layout_doc(
        r#"<html><body style="margin:0">
            <table><tr>
                <td id="cell" style="display:block; margin:0 auto; max-width:580px">
                    <div id="content" style="height:40px">body</div>
                </td>
            </tr></table>
        </body></html>"#,
    );
    let content = doc.query_selector("#content").unwrap().unwrap();
    let size = doc.get_node(content).unwrap().final_layout().size;
    assert_eq!(size.height, 40.0);
    assert!(size.width > 0.0, "{size:?}");
}
