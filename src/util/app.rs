use crate::util::StatefulList::StatefulList;

pub struct App<'a> {
    pub item_list: StatefulList<&'a str>,
}