use crate::s;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum EntryType {
    Article,
    Book,
    MvBook,
    InBook,
    BookInBook,
    SuppBook,
    Booklet,
    Collection,
    MvCollection,
    InCollection,
    SuppCollection,
    Manual,
    Misc,
    Online,
    Patent,
    Periodical,
    SuppPeriodical,
    Proceedings,
    MvProceedings,
    InProceedings,
    Reference,
    MvReference,
    InReference,
    Report,
    Set,
    Thesis,
    Unpublished,
    Custom,
    Conference,
    Electronic,
    MasterThesis,
    PhdThesis,
    TechReport,
    DataType,
    Other(String),
}

impl EntryType {
    pub(crate) fn from_str(s: &str) -> EntryType {
        match s {
            "article" => EntryType::Article,
            "book" => EntryType::Book,
            "mvbook" => EntryType::MvBook,
            "inbook" => EntryType::InBook,
            "bookinbook" => EntryType::BookInBook,
            "suppbook" => EntryType::SuppBook,
            "booklet" => EntryType::Booklet,
            "collection" => EntryType::Collection,
            "mvcollection" => EntryType::MvCollection,
            "incollection" => EntryType::InCollection,
            "suppcollection" => EntryType::SuppCollection,
            "manual" => EntryType::Manual,
            "misc" => EntryType::Misc,
            "online" => EntryType::Online,
            "patent" => EntryType::Patent,
            "periodical" => EntryType::Periodical,
            "suppperiodical" => EntryType::SuppPeriodical,
            "proceedings" => EntryType::Proceedings,
            "mvproceedings" => EntryType::MvProceedings,
            "inproceedings" => EntryType::InProceedings,
            "reference" => EntryType::Reference,
            "mvreference" => EntryType::MvReference,
            "inreference" => EntryType::InReference,
            "report" => EntryType::Report,
            "set" => EntryType::Set,
            "thesis" => EntryType::Thesis,
            "unpublished" => EntryType::Unpublished,
            "custom" => EntryType::Custom,
            "conference" => EntryType::Conference,
            "electronic" => EntryType::Electronic,
            "masterthesis" => EntryType::MasterThesis,
            "phdthesis" => EntryType::PhdThesis,
            "techreport" => EntryType::TechReport,
            "datatype" => EntryType::DataType,
            _ => EntryType::Other(s!(s)),
        }
    }
}

impl std::fmt::Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryType::Other(s) => write!(f, "{}", s.to_lowercase()),
            _ => write!(f, "{}", format!("{:?}", self).to_lowercase()),
        }
    }
}
