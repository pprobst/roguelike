use super::{Colors, Colorscheme};
use std::collections::HashMap;

pub struct RawColors {
    pub raws: Colors,
    pub color_index: HashMap<String, usize>,
    curr_colorscheme: usize,
}

impl RawColors {
    pub fn empty() -> Self {
        RawColors {
            raws: Colors {
                colorschemes: Vec::new(),
            },
            color_index: HashMap::new(),
            curr_colorscheme: 0,
        }
    }

    pub fn load(&mut self, raws: Colors) {
        self.raws = raws;
        for (i, color) in self.raws.colorschemes.iter().enumerate() {
            self.color_index.insert(color.name.clone(), i);
        }
    }

    pub fn set_curr_colorscheme(&mut self, colorscheme: &str) {
        self.curr_colorscheme = self.color_index[colorscheme];
    }

    pub fn get_curr_colorscheme(&self) -> &Colorscheme {
        &self.raws.colorschemes[self.curr_colorscheme]
    }
}
