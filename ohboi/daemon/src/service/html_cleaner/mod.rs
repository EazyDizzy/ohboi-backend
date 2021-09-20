use std::ops::Deref;

use regex::Regex;
use serde::Deserialize;

lazy_static! {
    static ref LINK_REGEX: Regex = Regex::new(r"(?i)<a[^>]+>(.+?)</a>").unwrap();
    static ref HTML_SPACES_REGEX: Regex = Regex::new(r"(?im)>[ ]+<").unwrap();
    static ref SPACES_REGEX: Regex = Regex::new(r"(?im)[ ]{2,}").unwrap();
    static ref SPEC_SYMBOLS_MAPPING_TYPED: Vec<SpecialSymbolMapping> =
        serde_json::from_str(include_str!("spec_symbols_mapping.json")).unwrap();
    static ref NEW_LINES_REGEX: Regex = Regex::new(r"(?im)[\n\t]").unwrap();
    static ref SCRIPTS_AND_STYES_REGEX: Regex = Regex::new(r"(?im)>[^<]+</(script|style)").unwrap();
    static ref HTML_TAGS_REGEX: Regex = Regex::new(r"(?im)<[^>]*>").unwrap();
}
// TODO remove empty tags
pub fn inner_text(html: &str) -> String {
    let no_new_lines_html = NEW_LINES_REGEX.replace_all(html, "").to_string();
    let no_scripts_html = SCRIPTS_AND_STYES_REGEX
        .replace_all(&no_new_lines_html, "")
        .to_string();
    let no_html_tags = HTML_TAGS_REGEX
        .replace_all(&no_scripts_html, "")
        .to_string();

    clean_html(&no_html_tags).trim().to_string()
}
pub fn clean_html(html: &str) -> String {
    remove_multiple_spaces(&replace_html_entities(remove_unneeded_tags(html)))
}

fn replace_html_entities(mut html: String) -> String {
    for symbol_mapping in SPEC_SYMBOLS_MAPPING_TYPED.deref() {
        let names_list = symbol_mapping.named.split(' ');

        for name in names_list {
            html = html.replace(name, &symbol_mapping.symbol);
        }
    }

    html
}

fn remove_multiple_spaces(html: &str) -> String {
    SPACES_REGEX
        .replace_all(HTML_SPACES_REGEX.replace_all(html, "><").as_ref(), "")
        .to_string()
}

fn remove_unneeded_tags(html: &str) -> String {
    let html_without_indents = html.replace("\n", "").replace("\t", "");

    let mut clean_html = html_without_indents.clone();
    for capture in LINK_REGEX.captures_iter(&html_without_indents) {
        clean_html = clean_html.replace(
            capture.get(0).unwrap().as_str(),
            capture.get(1).unwrap().as_str(),
        );
    }

    clean_html
}

#[derive(Debug, Deserialize)]
pub struct SpecialSymbolMapping {
    symbol: String,
    named: String,
}

#[cfg(test)]
mod tests {
    use crate::service::html_cleaner::{
        clean_html, inner_text, remove_multiple_spaces, remove_unneeded_tags, replace_html_entities,
    };

    #[test]
    fn it_removes_new_lines() {
        assert_eq!(
            remove_unneeded_tags("<div>\n</div>"),
            "<div></div>".to_string()
        );
    }

    #[test]
    fn it_removes_tabs() {
        assert_eq!(
            remove_unneeded_tags("<div>her\ther</div>"),
            "<div>herher</div>".to_string()
        );
    }

    #[test]
    fn it_removes_simple_links() {
        assert_eq!(remove_unneeded_tags("<div><a href=\"https://trello.com/c/1HiOMiAR/72-clean-product-description\">Link text</a></div>"), "<div>Link text</div>".to_string());
    }

    #[test]
    fn it_removes_links_with_attributes() {
        assert_eq!(remove_unneeded_tags("<div><a href=\"https://trello.com/c/1HiOMiAR/72-clean-product-description\" target=\"_blank\" type=\"her\">Link text</a></div>"), "<div>Link text</div>".to_string());
    }

    #[test]
    fn it_removes_links_with_nested_content() {
        assert_eq!(remove_unneeded_tags("<a href=\"https://www.w3schools.com\"><img border=\"0\" alt=\"W3Schools\" src=\"logo_w3s.gif\"></a>"), "<img border=\"0\" alt=\"W3Schools\" src=\"logo_w3s.gif\">".to_string());
    }

    #[test]
    fn it_replaces_non_breaking_spaces() {
        assert_eq!(
            replace_html_entities("<p>&nbsp;&nbsp;da&nbsp;</p>".to_string()),
            "<p>  da </p>".to_string()
        );
    }

    #[test]
    fn it_replaces_gt_and_lt() {
        assert_eq!(
            replace_html_entities("<p>e33 dick &gt; dendi dick</p>".to_string()),
            "<p>e33 dick > dendi dick</p>".to_string()
        );
        assert_eq!(
            replace_html_entities("<p>e33 ass &lt; dendi ass</p>".to_string()),
            "<p>e33 ass < dendi ass</p>".to_string()
        );
    }

    #[test]
    fn it_replaces_money() {
        assert_eq!(
            replace_html_entities("1&yen; = 1&cent; = 0.03&euro;".to_string()),
            "1¥ = 1¢ = 0.03€".to_string()
        );
    }

    #[test]
    fn it_removes_only_spaces_between_tags() {
        assert_eq!(
            remove_multiple_spaces("<p> h </p>      <p>e</>"),
            "<p> h </p><p>e</>".to_string()
        );
    }

    #[test]
    fn it_cleans_html() {
        assert_eq!(clean_html("<div><a href=\"https://trello.com/c/1HiOMiAR/72-clean-product-description\" target=\"_blank\" type=\"her\">Link text</a><p>\n&DownLeftVector;\t&uharr;</p></div>"), "<div>Link text<p>↽↾</p></div>".to_string());
    }

    #[test]
    fn it_removes_html_tags() {
        assert_eq!(inner_text(" <span>text</span>"), "text".to_string());
        assert_eq!(
            inner_text(" <span>Версия MIUI:</span>"),
            "Версия MIUI:".to_string()
        );
        assert_eq!(
            inner_text(
                r" <span>Ширина (мм):</span>
                                                "
            ),
            "Ширина (мм):".to_string()
        );
    }
}
