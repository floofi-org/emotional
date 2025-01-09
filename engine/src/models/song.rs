use crate::features::state::get_state;
use crate::utils::hash_text_color;
use web_sys::HtmlElement;
use crate::models::directory::DirectoryEntry;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub struct Song {
    pub versions: Vec<DirectoryEntry>,
    pub year: u32,
    pub edition: Vec<String>,
    pub artist: String,
    pub track: String,
    pub original: bool,
    pub ai: bool
}

impl Song {
    pub fn html(&self, id: usize, real_id: usize, prefix: &str) -> HtmlElement {
        let state = get_state();
        let document = &state.document;
        let element = document.create_element("a").unwrap();

        element.set_attribute("data-real-id", &real_id.to_string()).unwrap();
        element.set_id(&format!("{prefix}-item-{id}"));
        element.class_list().add_3("fella-list-item", "fella-list-link", "fella-list-item-padded")
            .unwrap();

        if !self.ai && !self.original {
            let artist = document.create_element("span").unwrap();
            artist.class_list().add_1("fella-footnotes").unwrap();
            artist.set_text_content(Some(&format!("{} - ", self.artist)));
            element.append_with_node_1(&artist).unwrap();
        }

        let track = document.create_element("span").unwrap();
        track.set_text_content(Some(&self.track));
        element.append_with_node_1(&track).unwrap();

        for ed in &self.edition {
            let edition = document.create_element("span").unwrap();
            edition.class_list().add_1("fella-badge-notice").unwrap();
            edition.set_text_content(Some(ed));
            let hash = hash_text_color(ed);
            edition.set_attribute("style",
                                  &format!("--fella-badge-notice-rgb: {},{},{} !important;", hash.0, hash.1, hash.2)
            ).unwrap();
            element.append_with_node_1(&edition).unwrap();
        }

        if self.ai {
            let badge = document.create_element("span").unwrap();
            badge.class_list().add_1("fella-badge-notice").unwrap();
            badge.set_text_content(Some("AI generated"));
            badge.set_attribute("style", "--fella-badge-notice-rgb: 255,161,212 !important;").unwrap();
            element.append_with_node_1(&badge).unwrap();
        }

        if self.original && !self.ai {
            let badge = document.create_element("span").unwrap();
            badge.class_list().add_1("fella-badge-notice").unwrap();
            badge.set_text_content(Some("Original"));
            badge.set_attribute("style", "--fella-badge-notice-rgb: 255,132,146 !important;").unwrap();
            element.append_with_node_1(&badge).unwrap();
        }

        if self.versions.len() > 1 {
            let versions = document.create_element("span").unwrap();
            versions.class_list().add_1("fella-badge-notice").unwrap();
            versions.set_text_content(Some(&format!("{} versions", self.versions.len())));
            let hash = hash_text_color(&self.versions.len().to_string());
            versions.set_attribute("style",
                                   &format!("--fella-badge-notice-rgb: {},{},{} !important;", hash.0, hash.1, hash.2)
            ).unwrap();
            element.append_with_node_1(&versions).unwrap();
        }

        let element: HtmlElement = element.dyn_into().unwrap();
        element
    }
}