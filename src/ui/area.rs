#[derive(Debug, Default, PartialEq)]
#[allow(unused)]
pub enum Area {
    #[default]
    Url,
    Collection,
    Request,
    Body,
}

impl Area {
    pub fn next(&self, expanded: bool) -> Self {
        match self {
            Area::Url => {
                if expanded {
                    Area::Collection
                } else {
                    Area::Request
                }
            }
            Area::Collection => Area::Request,
            Area::Request => Area::Body,
            Area::Body => Area::Url,
        }
    }

    pub fn previous(&self, expanded: bool) -> Self {
        match self {
            Area::Url => Area::Body,
            Area::Collection => Area::Url,
            Area::Request => {
                if expanded {
                    Area::Collection
                } else {
                    Area::Url
                }
            }
            Area::Body => Area::Request,
        }
    }
}
