use serde::{Deserialize, Serialize};

macro_rules! locales {
    ($($val:literal = $name:ident,)*) => {
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
        /// A users locale (representing their selected language)
        ///
        /// [Read More](https://discord.com/developers/docs/reference#locales)
        pub enum Locale {
            $(
                #[doc = concat!("The locale for ", stringify!($name), ", which represents ", $val, " in the discord api.")]
                #[serde(rename = $val)]
                $name,
            )*
        }
    };
}

locales![
    "da" = Danish,
    "de" = German,
    "en-GB" = EnglishUK,
    "en-US" = EnglishUS,
    "es-ES" = Spanish,
    "fr" = French,
    "hr" = Croatian,
    "it" = Italian,
    "lt" = Lithuanian,
    "hu" = Hungarian,
    "nl" = Dutch,
    "no" = Norwegian,
    "pl" = Polish,
    "pt-BR" = PortugueseBrazil,
    "ro" = Romanian,
    "fi" = Finnish,
    "sv-SE" = Swedish,
    "vi" = Vietnamese,
    "tr" = Turkish,
    "cz" = Czech,
    "el" = Greek,
    "bg" = Bulgarian,
    "ru" = Russian,
    "uk" = Ukrainian,
    "hi" = Hindi,
    "th" = Thai,
    "zh-CN" = ChineseChina,
    "ja" = Japanese,
    "zh-TW" = ChineseTaiwan,
    "ko" = Korean,
];

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::Locale::{self, *};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Test {
        pub locale: Locale,
    }

    #[test]
    fn eq() {
        assert_eq!(Russian, Russian);
        assert_ne!(Danish, Ukrainian);
        let x = Korean;
        assert_eq!(Korean, x);
    }

    #[test]
    fn serialize() {
        let my_struct = Test { locale: Russian };
        assert_eq!(
            serde_json::ser::to_string(&my_struct).unwrap(),
            "{\"locale\":\"ru\"}"
        );
    }

    #[test]
    fn deserialize() {
        let korean = "{\"locale\":\"ko\"}";
        assert_eq!(
            serde_json::from_str::<Test>(korean).unwrap(),
            Test { locale: Korean }
        );
        let polish = "{
            \"locale\": \"pl\"
        }";
        assert_eq!(serde_json::from_str::<Test>(polish).unwrap().locale, Polish)
    }
}
