use std::fmt::Display;

use crate::props::ArcDir;
use crate::Props;
impl Display for Props {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let directory = match self.element_type {
            ArcDir::Directory => true,
            ArcDir::Archive(_) => false,
        };
        let read_only = if let ArcDir::Archive(arc_props) = self.element_type {
            arc_props.read_only
        } else {
            false
        };
        write!(
            f,
            "{}{}{}{}{}{}",
            if directory { 'd' } else { '-' },
            if !directory { 'a' } else { '-' },
            if read_only { 'r' } else { '-' },
            if self.hidden { 'h' } else { '-' },
            if self.system { 's' } else { '-' },
            if self.reparse { 'l' } else { '-' }
        )
    }
}
