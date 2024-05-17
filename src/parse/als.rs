use crate::parse::{group, track};
use roxmltree::Document;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AlsData {
    pub groups: Vec<group::Group>,
    pub tracks: Vec<track::Track>,
}

impl AlsData {
    pub fn parse(xml_contents: String) -> AlsData {
        let doc = Document::parse(&xml_contents).unwrap();
        let root = doc.root_element();

        let tracks: Vec<track::Track> = root
            .descendants()
            .filter(|n| {
                n.has_tag_name("MidiTrack")
                    || n.has_tag_name("AudioTrack")
                    || n.has_tag_name("ReturnTrack")
            })
            .map(|n| track::Track::parse(n))
            .collect();

        let groups: Vec<group::Group> = root
            .descendants()
            .filter_map(|n| group::Group::parse(n))
            .collect();

        return AlsData { groups, tracks };
    }
}