use crate::date::Date;
use crate::edition::Edition;
use crate::pages::Pages;
use crate::person::Person;
use crate::s;

#[derive(Debug, PartialEq)]
pub(crate) enum EntryField {
    Abstract(String),
    Afterword(String),
    Annotation(String),
    Annotator(Vec<Person>),
    Author(Vec<Person>),
    AuthorType(String),
    BookAuthor(Vec<Person>),
    BookPagination(String),
    BookSubtitle(String),
    Chapter(String),
    Commentator(Vec<Person>),
    Date(Date),
    Doi(String),
    Edition(Edition),
    Editor(Vec<Person>),
    EditorType(String),
    Eid(String),
    EntrySubtype,
    EPrint(String),
    EPrintType(String),
    EPrintClass(String),
    EventDate(Date),
    EventTitle(String),
    File(String),
    Foreword(String),
    Holder(Vec<Person>),
    HowPublished(String),
    IndexTitle(String),
    Institution(String),
    Introduction(String),
    Isan(String),
    Isbn(String),
    Ismn(String),
    Isrn(String),
    Issue(String),
    IssueSubtitle(String),
    IssueTitle(String),
    Iswc(String),
    JournalSubtitle(String),
    JournalTitle(String),
    Label(String),
    Language(String),
    Library(String),
    Location(String),
    MainSubtitle(String),
    MainTitle(String),
    Month(Date),
    Note(String),
    Number(String),
    Organization(String),
    OrigDate(Date),
    OrigLanguage(String),
    OrigLocation(String),
    OrigPublisher(String),
    OrigTitle(String),
    Pages(Vec<Pages>),
    PageTotal(u32),
    Pagination(String),
    Part(String),
    Publisher(String),
    PubState(String),
    ReprintTitle(String),
    Series(String),
    ShortAuthor(String),
    ShortEdition(String),
    Shorthand(String),
    ShorthandIntro(String),
    ShortJournal(String),
    ShortSeries(String),
    ShortTitle(String),
    Subtitle(String),
    Title(String),
    Translator(Vec<Person>),
    Type(String),
    Url(String),
    UrlDate(Date),
    Venue(String),
    Version(String),
    Volume(String),
    Year(Date),
    Other(String),
}

impl EntryField {
    pub(crate) fn from_field_name_and_value(
        field_name: &str,
        value: &str,
    ) -> Result<EntryField, String> {
        let entry_field = match field_name {
            "abstract" => EntryField::Abstract(s!(value)),
            "afterword" => EntryField::Afterword(s!(value)),
            "annotation" => EntryField::Annotation(s!(value)),
            "annotator" => EntryField::Annotator(Person::people_from_str(value)?),
            "author" => EntryField::Author(Person::people_from_str(value)?),
            "authortype" => EntryField::AuthorType(s!(value)),
            "bookauthor" => EntryField::BookAuthor(Person::people_from_str(value)?),
            "bookpagination" => EntryField::BookPagination(s!(value)),
            "booksubtitle" => EntryField::BookSubtitle(s!(value)),
            "booktitle" => EntryField::BookSubtitle(s!(value)),
            "chapter" => EntryField::Chapter(s!(value)),
            "commentator" => EntryField::Commentator(Person::people_from_str(value)?),
            "date" => EntryField::Date(Date::parse_date_from_str(value)?),
            "doi" => EntryField::Doi(s!(value)),
            "edition" => EntryField::Edition(Edition::parse(value)),
            "editor" => EntryField::Editor(Person::people_from_str(value)?),
            "editortype" => EntryField::EditorType(s!(value)),
            "eid" => EntryField::Eid(s!(value)),
            "entrysubtype" => EntryField::EntrySubtype,
            "eprint" => EntryField::EPrint(s!(value)),
            "eprinttype" => EntryField::EPrintType(s!(value)),
            "eprintclass" => EntryField::EPrintClass(s!(value)),
            "eventdate" => EntryField::EventDate(Date::parse_date_from_str(value)?),
            "eventtitle" => EntryField::EventTitle(s!(value)),
            "file" => EntryField::File(s!(value)),
            "foreword" => EntryField::Foreword(s!(value)),
            "holder" => EntryField::Holder(Person::people_from_str(value)?),
            "howpublished" => EntryField::HowPublished(s!(value)),
            "indextitle" => EntryField::IndexTitle(s!(value)),
            "institution" => EntryField::Institution(s!(value)),
            "introduction" => EntryField::Introduction(s!(value)),
            "isan" => EntryField::Isan(s!(value)),
            "isbn" => EntryField::Isbn(s!(value)),
            "ismn" => EntryField::Ismn(s!(value)),
            "isrn" => EntryField::Isrn(s!(value)),
            "issue" => EntryField::Issue(s!(value)),
            "issuesubtitle" => EntryField::IssueSubtitle(s!(value)),
            "issuetitle" => EntryField::IssueTitle(s!(value)),
            "iswc" => EntryField::Iswc(s!(value)),
            "journalsubtitle" => EntryField::JournalSubtitle(s!(value)),
            "journaltitle" => EntryField::JournalTitle(s!(value)),
            "label" => EntryField::Label(s!(value)),
            "language" => EntryField::Language(s!(value)),
            "library" => EntryField::Library(s!(value)),
            "location" => EntryField::Location(s!(value)),
            "mainsubtitle" => EntryField::MainSubtitle(s!(value)),
            "maintitle" => EntryField::MainTitle(s!(value)),
            "month" => EntryField::Month(Date::parse_month_from_str(value)?),
            "note" => EntryField::Note(s!(value)),
            "number" => EntryField::Number(s!(value)),
            "organization" => EntryField::Organization(s!(value)),
            "origdate" => EntryField::OrigDate(Date::parse_date_from_str(value)?),
            "origlanguage" => EntryField::OrigLanguage(s!(value)),
            "origlocation" => EntryField::OrigLocation(s!(value)),
            "origpublisher" => EntryField::OrigPublisher(s!(value)),
            "origtitle" => EntryField::OrigTitle(s!(value)),
            "pages" => EntryField::Pages(Pages::pages_from_str(value)),
            "pagetotal" => EntryField::PageTotal(
                value
                    .parse()
                    .map_err(|_| format!("Could not parse PageTotal value from '{}'", value))?,
            ),
            "pagination" => EntryField::Pagination(s!(value)),
            "part" => EntryField::Part(s!(value)),
            "publisher" => EntryField::Publisher(s!(value)),
            "pubstate" => EntryField::PubState(s!(value)),
            "reprinttitle" => EntryField::ReprintTitle(s!(value)),
            "series" => EntryField::Series(s!(value)),
            "shortauthor" => EntryField::ShortAuthor(s!(value)),
            "shortedition" => EntryField::ShortEdition(s!(value)),
            "shorthand" => EntryField::Shorthand(s!(value)),
            "shorthandintro" => EntryField::ShorthandIntro(s!(value)),
            "shortjournal" => EntryField::ShortJournal(s!(value)),
            "shortseries" => EntryField::ShortSeries(s!(value)),
            "shorttitle" => EntryField::ShortTitle(s!(value)),
            "subtitle" => EntryField::Subtitle(s!(value)),
            "title" => EntryField::Title(s!(value)),
            "translator" => EntryField::Translator(Person::people_from_str(value)?),
            "type" => EntryField::Type(s!(value)),
            "url" => EntryField::Url(s!(value)),
            "urldate" => EntryField::UrlDate(Date::parse_date_from_str(value)?),
            "venue" => EntryField::Venue(s!(value)),
            "version" => EntryField::Version(s!(value)),
            "volume" => EntryField::Volume(s!(value)),
            "year" => EntryField::Year(Date::parse_year_from_str(value)?),
            _ => EntryField::Other(s!(field_name)),
        };

        Ok(entry_field)
    }
}

impl std::fmt::Display for EntryField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryField::Other(s) => write!(f, "{}", s.to_lowercase()),
            _ => write!(f, "{}", format!("{:?}", self).to_lowercase()),
        }
    }
}
