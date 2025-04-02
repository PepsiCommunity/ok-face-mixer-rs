use image::{ColorType, DynamicImage, GenericImage, GenericImageView, ImageReader};
use lazy_static::lazy_static;
use log::debug;

use crate::{Smile, SmileType};

pub struct SmileElement {
    image: DynamicImage,
    pos: (usize, usize),
}

impl SmileElement {
    pub fn new(image_name: &str, pos: (usize, usize)) -> Self {
        debug!("loading image {}", image_name);

        Self {
            image: ImageReader::open(image_name)
                .expect("image reading error")
                .decode()
                .expect("image format error"),
            pos,
        }
    }
}

pub type OptionalSmileElement = Option<&'static SmileElement>;

lazy_static! {
    pub static ref BACKGROUND: SmileElement = SmileElement::new("res/background.png", (0, 0));
    pub static ref GRIN_EYES: SmileElement = SmileElement::new("res/eyes/grin.png", (9, 2));
    pub static ref GRIN_MOUTH: SmileElement = SmileElement::new("res/mouth/grin.png", (0, 25));
    pub static ref ANGRY_EYES: SmileElement = SmileElement::new("res/eyes/angry.png", (10, 6));
    pub static ref ANGRY_MOUTH: SmileElement = SmileElement::new("res/mouth/angry.png", (7, 39));
    pub static ref FLUSH_BROWS: SmileElement = SmileElement::new("res/brows/flush.png", (8, 2));
    pub static ref FLUSH_EYES: SmileElement = SmileElement::new("res/eyes/flush.png", (4, 10));
    pub static ref FLUSH_MOUTH: SmileElement = SmileElement::new("res/mouth/flush.png", (22, 43));
    pub static ref HE_BROWS: SmileElement = SmileElement::new("res/brows/he.png", (9, 3));
    pub static ref HE_EYES: SmileElement = SmileElement::new("res/eyes/he.png", (1, 13));
    pub static ref HE_MOUTH: SmileElement = SmileElement::new("res/mouth/he.png", (2, 42));
    pub static ref MAD_BROWS: SmileElement = SmileElement::new("res/brows/mad.png", (13, 4));
    pub static ref MAD_EYES: SmileElement = SmileElement::new("res/eyes/mad.png", (11, 12));
    pub static ref MAD_MOUTH: SmileElement = SmileElement::new("res/mouth/mad.png", (2, 40));
    pub static ref PLEAD_BROWS: SmileElement = SmileElement::new("res/brows/plead.png", (7, 4));
    pub static ref PLEAD_EYES: SmileElement = SmileElement::new("res/eyes/plead.png", (10, 9));
    pub static ref PLEAD_MOUTH: SmileElement = SmileElement::new("res/mouth/plead.png", (18, 47));
    pub static ref SAD_BROWS: SmileElement = SmileElement::new("res/brows/sad.png", (12, 3));
    pub static ref SAD_EYES: SmileElement = SmileElement::new("res/eyes/sad.png", (10, 12));
    pub static ref SAD_MOUTH: SmileElement = SmileElement::new("res/mouth/sad.png", (3, 39));
    pub static ref SG_BROWS: SmileElement = SmileElement::new("res/brows/sg.png", (4, 2));
    pub static ref SG_EYES: SmileElement = SmileElement::new("res/eyes/sg.png", (8, 11));
    pub static ref SG_MOUTH: SmileElement = SmileElement::new("res/mouth/sg.png", (4, 44));
    pub static ref SHOCK_BROWS: SmileElement = SmileElement::new("res/brows/shock.png", (7, 3));
    pub static ref SHOCK_EYES: SmileElement = SmileElement::new("res/eyes/shock.png", (5, 9));
    pub static ref SHOCK_MOUTH: SmileElement = SmileElement::new("res/mouth/shock.png", (27, 52));
    pub static ref SL_SMILE_BROWS: SmileElement =
        SmileElement::new("res/brows/sl_smile.png", (10, 3));
    pub static ref SL_SMILE_EYES: SmileElement =
        SmileElement::new("res/eyes/sl_smile.png", (9, 11));
    pub static ref SL_SMILE_MOUTH: SmileElement =
        SmileElement::new("res/mouth/sl_smile.png", (3, 42));
    pub static ref SLEEP_BROWS: SmileElement = SmileElement::new("res/brows/sleep.png", (7, 4));
    pub static ref SLEEP_EYES: SmileElement = SmileElement::new("res/eyes/sleep.png", (9, 11));
    pub static ref SLEEP_MOUTH: SmileElement = SmileElement::new("res/mouth/sleep.png", (29, 50));
    pub static ref SMILEY_BROWS: SmileElement = SmileElement::new("res/brows/smiley.png", (11, 1));
    pub static ref SMILEY_EYES: SmileElement = SmileElement::new("res/eyes/smiley.png", (11, 4));
    pub static ref SMILEY_MOUTH: SmileElement = SmileElement::new("res/mouth/smiley.png", (1, 30));
    pub static ref TONG_BROWS: SmileElement = SmileElement::new("res/brows/tong.png", (8, 4));
    pub static ref TONG_EYES: SmileElement = SmileElement::new("res/eyes/tong.png", (9, 10));
    pub static ref TONG_MOUTH: SmileElement = SmileElement::new("res/mouth/tong.png", (3, 31));
    pub static ref UNAMUS_BROWS: SmileElement = SmileElement::new("res/brows/unamus.png", (16, 4));
    pub static ref UNAMUS_EYES: SmileElement = SmileElement::new("res/eyes/unamus.png", (8, 10));
    pub static ref UNAMUS_MOUTH: SmileElement = SmileElement::new("res/mouth/unamus.png", (5, 44));
    pub static ref WINK_BROWS: SmileElement = SmileElement::new("res/brows/wink.png", (14, 4));
    pub static ref WINK_EYES: SmileElement = SmileElement::new("res/eyes/wink.png", (8, 14));
    pub static ref WINK_MOUTH: SmileElement = SmileElement::new("res/mouth/wink.png", (6, 35));
    pub static ref ZANY_BROWS: SmileElement = SmileElement::new("res/brows/zany.png", (14, 1));
    pub static ref ZANY_EYES: SmileElement = SmileElement::new("res/eyes/zany.png", (11, 8));
    pub static ref ZANY_MOUTH: SmileElement = SmileElement::new("res/mouth/zany.png", (26, 51));
}

pub const SMILE_WIDTH: u32 = 64;
pub const SMILE_HEIGHT: u32 = 64;

impl Smile {
    pub fn generate(&self) -> DynamicImage {
        debug!("generating image...");

        let (brows, eyes, mouth) = self.get_parts();
        let mut image = DynamicImage::new(SMILE_WIDTH, SMILE_HEIGHT, ColorType::Rgb8);

        Self::add_element(&mut image, &Some(&BACKGROUND));
        Self::add_element(&mut image, &brows);
        Self::add_element(&mut image, &eyes);
        Self::add_element(&mut image, &mouth);

        image
    }

    pub fn get_parts(
        &self,
    ) -> (
        OptionalSmileElement,
        OptionalSmileElement,
        OptionalSmileElement,
    ) {
        (
            match self.left {
                SmileType::Grin => None,
                SmileType::Angry => None,
                SmileType::Flush => Some(&FLUSH_BROWS),
                SmileType::He => Some(&HE_BROWS),
                SmileType::Mad => Some(&MAD_BROWS),
                SmileType::Plead => Some(&PLEAD_BROWS),
                SmileType::Sad => Some(&SAD_BROWS),
                SmileType::Sg => Some(&SG_BROWS),
                SmileType::Shock => Some(&SHOCK_BROWS),
                SmileType::SlSmile => Some(&SL_SMILE_BROWS),
                SmileType::Sleep => Some(&SLEEP_BROWS),
                SmileType::Smiley => Some(&SMILEY_BROWS),
                SmileType::Tong => Some(&TONG_BROWS),
                SmileType::Unamus => Some(&UNAMUS_BROWS),
                SmileType::Wink => Some(&WINK_BROWS),
                SmileType::Zany => Some(&ZANY_BROWS),
            },
            match self.left {
                SmileType::Grin => Some(&GRIN_EYES),
                SmileType::Angry => Some(&ANGRY_EYES),
                SmileType::Flush => Some(&FLUSH_EYES),
                SmileType::He => Some(&HE_EYES),
                SmileType::Mad => Some(&MAD_EYES),
                SmileType::Plead => Some(&PLEAD_EYES),
                SmileType::Sad => Some(&SAD_EYES),
                SmileType::Sg => Some(&SG_EYES),
                SmileType::Shock => Some(&SHOCK_EYES),
                SmileType::SlSmile => Some(&SL_SMILE_EYES),
                SmileType::Sleep => Some(&SLEEP_EYES),
                SmileType::Smiley => Some(&SMILEY_EYES),
                SmileType::Tong => Some(&TONG_EYES),
                SmileType::Unamus => Some(&UNAMUS_EYES),
                SmileType::Wink => Some(&WINK_EYES),
                SmileType::Zany => Some(&ZANY_EYES),
            },
            match self.right {
                SmileType::Grin => Some(&GRIN_MOUTH),
                SmileType::Angry => Some(&ANGRY_MOUTH),
                SmileType::Flush => Some(&FLUSH_MOUTH),
                SmileType::He => Some(&HE_MOUTH),
                SmileType::Mad => Some(&MAD_MOUTH),
                SmileType::Plead => Some(&PLEAD_MOUTH),
                SmileType::Sad => Some(&SAD_MOUTH),
                SmileType::Sg => Some(&SG_MOUTH),
                SmileType::Shock => Some(&SHOCK_MOUTH),
                SmileType::SlSmile => Some(&SL_SMILE_MOUTH),
                SmileType::Sleep => Some(&SLEEP_MOUTH),
                SmileType::Smiley => Some(&SMILEY_MOUTH),
                SmileType::Tong => Some(&TONG_MOUTH),
                SmileType::Unamus => Some(&UNAMUS_MOUTH),
                SmileType::Wink => Some(&WINK_MOUTH),
                SmileType::Zany => Some(&ZANY_MOUTH),
            },
        )
    }

    fn add_element(img: &mut DynamicImage, el: &OptionalSmileElement) {
        if let Some(el) = *el {
            let pixel_pos: Vec<_> = el
                .image
                .pixels()
                .map(|x| (x.0 + el.pos.0 as u32, x.1 + el.pos.1 as u32))
                .collect();

            img.clone().pixels().for_each(|x| {
                img.put_pixel(
                    x.0,
                    x.1,
                    if pixel_pos.contains(&(x.0, x.1))
                        && el
                            .image
                            .get_pixel(x.0 - el.pos.0 as u32, x.1 - el.pos.1 as u32)
                            .0
                            != [0; 4]
                    {
                        el.image
                            .get_pixel(x.0 - el.pos.0 as u32, x.1 - el.pos.1 as u32)
                    } else {
                        x.2
                    },
                )
            });
        }
    }
}
