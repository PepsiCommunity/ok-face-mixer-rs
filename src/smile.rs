use std::str::FromStr;

use image::{ColorType, DynamicImage, GenericImage, GenericImageView, ImageReader};
use lazy_static::lazy_static;
use log::debug;

pub struct SmileElement {
    image: DynamicImage,
    pos: (usize, usize),
}

pub type OptionalSmileElement = Option<&'static SmileElement>;

pub struct Smile {
    brow: OptionalSmileElement,
    eye: OptionalSmileElement,
    mouth: OptionalSmileElement,
}

lazy_static! {
    pub static ref BACKGROUND: SmileElement = SmileElement::new("res/background.png", (0, 0));
    pub static ref GRIN_EYES: SmileElement = SmileElement::new("res/eyes/grin.png", (9, 2));
    pub static ref GRIN_MOUTH: SmileElement = SmileElement::new("res/mouth/grin.png", (0, 25));
    pub static ref ANGRY_EYES: SmileElement = SmileElement::new("res/eyes/angry.png", (10, 6));
    pub static ref ANGRY_MOUTH: SmileElement = SmileElement::new("res/mouth/angry.png", (7, 39));
    pub static ref FLUSH_BROWS: SmileElement = SmileElement::new("res/eyes/flush.png", (8, 2));
    pub static ref FLUSH_EYES: SmileElement = SmileElement::new("res/eyes/flush.png", (4, 10));
    pub static ref FLUSH_MOUTH: SmileElement = SmileElement::new("res/mouth/flush.png", (22, 43));
    pub static ref HE_BROWS: SmileElement = SmileElement::new("res/eyes/he.png", (9, 3));
    pub static ref HE_EYES: SmileElement = SmileElement::new("res/eyes/he.png", (1, 13));
    pub static ref HE_MOUTH: SmileElement = SmileElement::new("res/mouth/he.png", (2, 42));
    pub static ref MAD_BROWS: SmileElement = SmileElement::new("res/eyes/mad.png", (13, 4));
    pub static ref MAD_EYES: SmileElement = SmileElement::new("res/eyes/mad.png", (11, 12));
    pub static ref MAD_MOUTH: SmileElement = SmileElement::new("res/mouth/mad.png", (2, 40));
    pub static ref PLEAD_BROWS: SmileElement = SmileElement::new("res/eyes/plead.png", (7, 4));
    pub static ref PLEAD_EYES: SmileElement = SmileElement::new("res/eyes/plead.png", (10, 9));
    pub static ref PLEAD_MOUTH: SmileElement = SmileElement::new("res/mouth/plead.png", (18, 47));
    pub static ref SAD_BROWS: SmileElement = SmileElement::new("res/eyes/sad.png", (12, 3));
    pub static ref SAD_EYES: SmileElement = SmileElement::new("res/eyes/sad.png", (10, 12));
    pub static ref SAD_MOUTH: SmileElement = SmileElement::new("res/mouth/sad.png", (3, 39));
    pub static ref SG_BROWS: SmileElement = SmileElement::new("res/eyes/sg.png", (4, 2));
    pub static ref SG_EYES: SmileElement = SmileElement::new("res/eyes/sg.png", (8, 11));
    pub static ref SG_MOUTH: SmileElement = SmileElement::new("res/mouth/sg.png", (4, 44));
    pub static ref SHOCK_BROWS: SmileElement = SmileElement::new("res/eyes/shock.png", (7, 3));
    pub static ref SHOCK_EYES: SmileElement = SmileElement::new("res/eyes/shock.png", (5, 9));
    pub static ref SHOCK_MOUTH: SmileElement = SmileElement::new("res/mouth/shock.png", (27, 52));
    pub static ref SL_SMILE_BROWS: SmileElement =
        SmileElement::new("res/eyes/sl_smile.png", (10, 3));
    pub static ref SL_SMILE_EYES: SmileElement =
        SmileElement::new("res/eyes/sl_smile.png", (9, 11));
    pub static ref SL_SMILE_MOUTH: SmileElement =
        SmileElement::new("res/mouth/sl_smile.png", (3, 42));
    pub static ref SLEEP_BROWS: SmileElement = SmileElement::new("res/eyes/sleep.png", (7, 4));
    pub static ref SLEEP_EYES: SmileElement = SmileElement::new("res/eyes/sleep.png", (9, 11));
    pub static ref SLEEP_MOUTH: SmileElement = SmileElement::new("res/mouth/sleep.png", (29, 50));
    pub static ref SMILEY_BROWS: SmileElement = SmileElement::new("res/eyes/smiley.png", (11, 1));
    pub static ref SMILEY_EYES: SmileElement = SmileElement::new("res/eyes/smiley.png", (11, 4));
    pub static ref SMILEY_MOUTH: SmileElement = SmileElement::new("res/mouth/smiley.png", (1, 30));
    pub static ref TONG_BROWS: SmileElement = SmileElement::new("res/eyes/tong.png", (8, 4));
    pub static ref TONG_EYES: SmileElement = SmileElement::new("res/eyes/tong.png", (9, 10));
    pub static ref TONG_MOUTH: SmileElement = SmileElement::new("res/mouth/tong.png", (3, 31));
    pub static ref UNAMUS_BROWS: SmileElement = SmileElement::new("res/eyes/unamus.png", (16, 4));
    pub static ref UNAMUS_EYES: SmileElement = SmileElement::new("res/eyes/unamus.png", (8, 10));
    pub static ref UNAMUS_MOUTH: SmileElement = SmileElement::new("res/mouth/unamus.png", (5, 44));
    pub static ref WINK_BROWS: SmileElement = SmileElement::new("res/eyes/wink.png", (14, 4));
    pub static ref WINK_EYES: SmileElement = SmileElement::new("res/eyes/wink.png", (8, 14));
    pub static ref WINK_MOUTH: SmileElement = SmileElement::new("res/mouth/wink.png", (6, 35));
    pub static ref ZANY_BROWS: SmileElement = SmileElement::new("res/eyes/zany.png", (14, 1));
    pub static ref ZANY_EYES: SmileElement = SmileElement::new("res/eyes/zany.png", (11, 8));
    pub static ref ZANY_MOUTH: SmileElement = SmileElement::new("res/mouth/zany.png", (26, 51));
}

pub const SMILE_WIDTH: u32 = 64;
pub const SMILE_HEIGHT: u32 = 64;

impl Smile {
    #[inline]
    pub fn new(
        brow: OptionalSmileElement,
        eye: OptionalSmileElement,
        mouth: OptionalSmileElement,
    ) -> Self {
        Self { brow, eye, mouth }
    }

    #[inline]
    pub fn grin() -> Self {
        Self::new(None, Some(&GRIN_EYES), Some(&GRIN_MOUTH))
    }

    #[inline]
    pub fn angry() -> Self {
        Self::new(None, Some(&ANGRY_EYES), Some(&ANGRY_MOUTH))
    }

    #[inline]
    pub fn flush() -> Self {
        Self::new(Some(&FLUSH_BROWS), Some(&FLUSH_EYES), Some(&FLUSH_MOUTH))
    }

    #[inline]
    pub fn he() -> Self {
        Self::new(Some(&HE_BROWS), Some(&HE_EYES), Some(&HE_MOUTH))
    }

    #[inline]
    pub fn mad() -> Self {
        Self::new(Some(&MAD_BROWS), Some(&MAD_EYES), Some(&MAD_MOUTH))
    }

    #[inline]
    pub fn plead() -> Self {
        Self::new(Some(&PLEAD_BROWS), Some(&PLEAD_EYES), Some(&PLEAD_MOUTH))
    }

    #[inline]
    pub fn sad() -> Self {
        Self::new(Some(&SAD_BROWS), Some(&SAD_EYES), Some(&SAD_MOUTH))
    }

    #[inline]
    pub fn sg() -> Self {
        Self::new(Some(&SG_BROWS), Some(&SG_EYES), Some(&SG_MOUTH))
    }

    #[inline]
    pub fn shock() -> Self {
        Self::new(Some(&SHOCK_BROWS), Some(&SHOCK_EYES), Some(&SHOCK_MOUTH))
    }

    #[inline]
    pub fn sl_smile() -> Self {
        Self::new(
            Some(&SL_SMILE_BROWS),
            Some(&SL_SMILE_EYES),
            Some(&SL_SMILE_MOUTH),
        )
    }

    #[inline]
    pub fn sleep() -> Self {
        Self::new(Some(&SLEEP_BROWS), Some(&SLEEP_EYES), Some(&SLEEP_MOUTH))
    }

    #[inline]
    pub fn smiley() -> Self {
        Self::new(Some(&SMILEY_BROWS), Some(&SMILEY_EYES), Some(&SMILEY_MOUTH))
    }

    #[inline]
    pub fn tong() -> Self {
        Self::new(Some(&TONG_BROWS), Some(&TONG_EYES), Some(&TONG_MOUTH))
    }

    #[inline]
    pub fn unamus() -> Self {
        Self::new(Some(&UNAMUS_BROWS), Some(&UNAMUS_EYES), Some(&UNAMUS_MOUTH))
    }

    #[inline]
    pub fn wink() -> Self {
        Self::new(Some(&WINK_BROWS), Some(&WINK_EYES), Some(&WINK_MOUTH))
    }

    #[inline]
    pub fn zany() -> Self {
        Self::new(Some(&ZANY_BROWS), Some(&ZANY_EYES), Some(&ZANY_MOUTH))
    }

    #[inline]
    pub fn combine(left: Self, right: Self) -> Self {
        Self::new(left.brow, left.eye, right.mouth)
    }

    pub fn image(&self) -> DynamicImage {
        debug!("generating image...");

        let mut image = DynamicImage::new(SMILE_WIDTH, SMILE_HEIGHT, ColorType::Rgb8);

        Self::add_element(&mut image, &Some(&BACKGROUND));
        Self::add_element(&mut image, &self.brow);
        Self::add_element(&mut image, &self.eye);
        Self::add_element(&mut image, &self.mouth);

        image
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

    fn from_str_real(s: &str) -> Result<Self, ()> {
        match s {
            "grin" => Ok(Self::grin()),
            "angry" => Ok(Self::angry()),
            "flush" => Ok(Self::flush()),
            "he" => Ok(Self::he()),
            "mad" => Ok(Self::mad()),
            "plead" => Ok(Self::plead()),
            "sad" => Ok(Self::sad()),
            "sg" => Ok(Self::sg()),
            "shock" => Ok(Self::shock()),
            "sl_smile" => Ok(Self::sl_smile()),
            "sleep" => Ok(Self::sleep()),
            "smiley" => Ok(Self::smiley()),
            "tong" => Ok(Self::tong()),
            "unamus" => Ok(Self::unamus()),
            "wink" => Ok(Self::wink()),
            "zany" => Ok(Self::zany()),

            _ => Err(()),
        }
    }
}

impl FromStr for Smile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_real(s.replace("/json/", "").replace(".json", "").as_str())
    }
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
