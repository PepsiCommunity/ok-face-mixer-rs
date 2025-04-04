#[cfg(feature = "rand")]
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};
#[cfg(feature = "rand")]
use strum::IntoEnumIterator;

use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, PartialEq, Eq, Clone, EnumString, Display, EnumIter)]
#[strum(serialize_all = "snake_case")]
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

#[cfg(feature = "rand")]
impl Distribution<SmileType> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SmileType {
        SmileType::iter()
            .get(rng.random_range(..SmileType::iter().len()))
            .unwrap()
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
        assert_eq!(
            SmileType::Grin,
            SmileType::from_str("/json/grin.json").unwrap()
        );
        assert_eq!(SmileType::Zany, SmileType::from_str("zany").unwrap());
        assert_eq!(
            SmileType::Zany,
            SmileType::from_str("/json/zany.json").unwrap()
        );
    }

    #[test]
    fn api_query_test() {
        assert_eq!(
            "?left=grin&right=angry",
            Smile::new(SmileType::Grin, SmileType::Angry).api_query()
        );
    }
}
