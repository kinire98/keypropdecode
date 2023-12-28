use std::fmt::Display;

use crate::Props;
impl Display for Props {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}{}{}{}",
        if self.directory {'d'} else {'-'},
        if self.archive {'a'} else {'-'},
        if self.read_only {'r'} else {'-'},
        if self.hidden {'h'} else {'-'},
        if self.system {'s'} else {'-'},
        if self.reparse {'l'} else {'-'})
    }
}