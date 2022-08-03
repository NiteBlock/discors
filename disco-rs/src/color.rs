/// An alias for all the brits out there.
pub type Colour = Color;

/// Represents a Discord color. This is similar to storing information in an rgb tuple like, `(u8, u8, u8)`
///
/// Note: an alias exsits: [`Colour`] and for all methods there is an alternative spelling avalible for convience.
#[derive(Debug, Hash, PartialEq, Clone, Copy, PartialOrd)]
pub struct Color(pub u32);

macro_rules! colors {
    ($($color:ident $($alias:ident)* = $val:expr $(; $notes:expr)?,)*) => {
        impl Color {
            $(
                #[doc = concat!(
                    "A default color `",
                    stringify!($color),
                    "` that represents the color value of `",
                    stringify!($val),
                    "`. ",
                    $(
                        $notes,
                    )?
                )]
                pub const $color: Color = Color($val);

                $(
                    #[doc = concat!(
                        "An alias to the color `",
                        stringify!($color),
                        "`."
                    )]
                    pub const $alias: Color = Color::$color;
                )*
            )*
        }
    };
}

colors! {
    TEAL = 0x1abc9c,
    DARK_TEAL = 0x11806a,
    GREEN = 0x2ecc71,
    DARK_GREEN = 0x1f8b4c,
    BLUE = 0x3498db,
    DARK_BLUE = 0x206694,
    PURPLE = 0x9b59b6,
    DARK_PURPLE = 0x71368a,
    MAGENTA = 0xe91e63,
    DARK_MAGENTA = 0xad1457,
    GOLD = 0xf1c40f,
    DARK_GOLD = 0xc27c0e,
    ORANGE = 0xe67e22,
    DARK_ORANGE = 0xa84300,
    RED = 0xe74c3c,
    DARK_RED = 0x992d22,
    LIGHTER_GREY LIGHTER_GRAY = 0x95a5a6,
    DARK_GREY DARK_GRAY = 0x607d8b,
    LIGHT_GREY LIGHT_GRAY = 0x979c9f,
    DARKER_GREY DARKER_GRAY = 0x546e7a,
    BLURPLE = 0x7289da; "The discord mix of blue and purple.",
    GREYPLE = 0x99aab5,
    DARK_THEME = 0x36393f; "In embeds, this makes the side appear invisible, when in dark mode.",
}

impl Color {
    /// Creates a new color using a correct value passed.
    ///
    /// ```rust
    /// use disco_rs::color::Color;
    /// // red
    /// let my_color = Color::new(0xff0000);
    ///
    /// assert_eq!(my_color.rgb(), (255, 0, 0))
    /// ```
    pub const fn new(color: u32) -> Self {
        // cant be rgba, technically we should
        // assert!(color < 2u32.pow(24));
        Self(color)
    }
    // rgb and one hopefully empty byte.
    const fn rgb_(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
    /// Gives the `red` value of the RGB code.
    pub const fn r(&self) -> u8 {
        self.rgb_()[1]
    }
    /// Gives the `green` value of the RGB code.
    pub const fn g(&self) -> u8 {
        self.rgb_()[2]
    }
    /// Gives the `blue` value of the RGB code.
    pub const fn b(&self) -> u8 {
        self.rgb_()[3]
    }
    /// Gives the `rgb` value as a tuple.
    pub const fn rgb(&self) -> (u8, u8, u8) {
        (self.r(), self.g(), self.b())
    }
    /// Converts a RGB code into a Color.
    ///
    /// ```rust
    /// use disco_rs::color::Color;
    /// // A nice dark red.
    /// let my_color = Color::from_rgb(123, 23, 10);
    /// // The same color but in hex
    /// assert_eq!(my_color.0, 0x7b170a)
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(u32::from_be_bytes([0, r, g, b]))
    }
    /// Converts a RGB code tuple into a Color.
    pub const fn from_rgb_tuple(tup: (u8, u8, u8)) -> Self {
        Self(u32::from_be_bytes([0, tup.0, tup.1, tup.2]))
    }
}

impl ToString for Color {
    /// Converts the color to a [`String`] as a hex code.
    ///
    /// ```rust
    /// use disco_rs::color::Color;
    ///
    /// let my_color = Color::new(0xabc123);
    ///
    /// assert_eq!(my_color.to_string(), "#abc123")
    /// ```
    fn to_string(&self) -> String {
        // evil bit hack
        format!("#{:0>6x}", self.0)
    }
}

impl Default for Color {
    /// Gives the default color, with value `0`, Black.
    fn default() -> Self {
        Self(0)
    }
}

#[cfg(test)]
mod test {
    use super::{Color, Colour};
    // constant testing as well!?!?!
    const MY_COL: Colour = Color::new(0x12bcde);
    const WHITE: Color = Colour::from_rgb(255, 255, 255);

    #[test]
    fn value() {
        assert_eq!(MY_COL.0, 0x12bcde);
        assert_eq!(WHITE.0, 0xffffff);
    }

    #[test]
    fn rgb() {
        // my cols rgb is 18, 188, 222
        // trust
        assert_eq!(MY_COL.rgb(), (18, 188, 222));
        // black and white
        assert_eq!(Color::default().rgb(), (0, 0, 0));
        assert_eq!(WHITE.rgb(), (255, 255, 255));
    }

    #[test]
    fn rgb_individual() {
        assert_eq!(MY_COL.r(), 18);
        assert_eq!(MY_COL.g(), 188);
        assert_eq!(MY_COL.b(), 222);
        assert_eq!(WHITE.r(), 255);
        assert_eq!(WHITE.g(), 255);
        assert_eq!(WHITE.b(), 255);
    }

    #[test]
    fn from_rgb() {
        assert_eq!(MY_COL, Colour::from_rgb(18, 188, 222));
        // 0xe74c3c in rgb is 231, 76, 60
        assert_eq!(Color::RED, Color::from_rgb_tuple((231, 76, 60)))
    }
}
