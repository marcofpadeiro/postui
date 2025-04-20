#[derive(Debug, Default, PartialEq)]
#[allow(unused)]
pub enum Area {
    #[default]
    Url,
    Collection,
    Request,
    Response,
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
            Area::Request => Area::Response,
            Area::Response => Area::Url,
        }
    }

    pub fn previous(&self, expanded: bool) -> Self {
        match self {
            Area::Url => Area::Response,
            Area::Collection => Area::Url,
            Area::Request => {
                if expanded {
                    Area::Collection
                } else {
                    Area::Url
                }
            }
            Area::Response => Area::Request,
        }
    }
}
