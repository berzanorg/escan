pub enum Tag {
    Earliest,
    Pending,
    Latest,
}

impl Tag {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Earliest => "earliest",
            Self::Pending => "pending",
            Self::Latest => "latest",
        }
    }
}


pub enum Sort {
    Asc,
    Desc,
}

impl Sort {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }
}


pub enum BlockType {
    Blocks,
    Uncles,
}

impl BlockType {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Blocks => "blocks",
            Self::Uncles => "uncles",
        }
    }
}