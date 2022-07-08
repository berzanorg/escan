/// Can be `Earliest`, `Pending`, or `Latest`
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


/// Can be `Asc`, or `Desc`
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


/// Can be: `Blocks`, or `Uncles`
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