use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub enum SmileType {
    Grin,
    Angry,
    Flush,
    He,
    Mad,
    Plead,
    Sad,
    Sg,
    Shock,
    SlSmile,
    Sleep,
    Smiley,
    Tong,
    Unamus,
    Wink,
    Zany,
}

impl Display for SmileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Grin => "grin",
                Self::Angry => "angry",
                Self::Flush => "flush",
                Self::He => "he",
                Self::Mad => "mad",
                Self::Plead => "plead",
                Self::Sad => "sad",
                Self::Sg => "sg",
                Self::Shock => "shock",
                Self::SlSmile => "sl_smile",
                Self::Sleep => "sleep",
                Self::Smiley => "smiley",
                Self::Tong => "tong",
                Self::Unamus => "unamus",
                Self::Wink => "wink",
                Self::Zany => "zany",
            }
        )
    }
}

impl FromStr for SmileType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.replace("/api/json/", "").replace(".json", "").as_str() {
            "grin" => Ok(Self::Grin),
            "angry" => Ok(Self::Angry),
            "flush" => Ok(Self::Flush),
            "he" => Ok(Self::He),
            "mad" => Ok(Self::Mad),
            "plead" => Ok(Self::Plead),
            "sad" => Ok(Self::Sad),
            "sg" => Ok(Self::Sg),
            "shock" => Ok(Self::Shock),
            "sl_smile" => Ok(Self::SlSmile),
            "sleep" => Ok(Self::Sleep),
            "smiley" => Ok(Self::Smiley),
            "tong" => Ok(Self::Tong),
            "unamus" => Ok(Self::Unamus),
            "wink" => Ok(Self::Wink),
            "zany" => Ok(Self::Zany),

            _ => Err(()),
        }
    }
}

pub struct Smile {
    pub left: SmileType,
    pub right: SmileType,
}

impl Smile {
    pub fn new(left: SmileType, right: SmileType) -> Self {
        Self { left, right }
    }
}

#[cfg(any(feature = "query", test))]
impl Smile {
    pub fn api_query(&self) -> String {
        format!("?left={}&right={}", self.left, self.right)
    }
}

#[cfg(feature = "generator")]
mod generator;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{Smile, SmileType};

    #[test]
    fn to_str_test() {
        assert_eq!("grin", SmileType::Grin.to_string());
        assert_eq!("angry", SmileType::Angry.to_string());
        assert_eq!("he", SmileType::He.to_string());
    }

    #[test]
    fn from_str_test() {
        assert_eq!(SmileType::Grin, SmileType::from_str("grin").unwrap());
        assert_eq!(SmileType::Grin, SmileType::from_str("/api/json/grin.json").unwrap());
        assert_eq!(SmileType::Zany, SmileType::from_str("zany").unwrap());
        assert_eq!(SmileType::Zany, SmileType::from_str("/api/json/zany.json").unwrap());
    }

    #[test]
    fn api_query_test() {
        assert_eq!("?left=grin&right=angry", Smile::new(SmileType::Grin, SmileType::Angry).api_query());
    }
}
