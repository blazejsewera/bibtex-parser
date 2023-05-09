use crate::entry_field::EntryField;
use crate::entry_type::EntryType;

struct Entry {
    r#type: EntryType,
    fields: Vec<EntryField>,
}
