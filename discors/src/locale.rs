use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Locale {
    #[serde(rename = "da")]
    Danish,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "en-GB")]
    EnglishUK,
    #[serde(rename = "en-US")]
    EnglishUS,
    #[serde(rename = "es-ES")]
    Spanish, // spain
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "hr")]
    Croatian,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "lt")]
    Lithuanian,
    #[serde(rename = "hu")]
    Hungarian,
    #[serde(rename = "nl")]
    Dutch,
    #[serde(rename = "no")]
    Norwegian,
    #[serde(rename = "pl")]
    Polish,
    #[serde(rename = "pt-BR")]
    PortugueseBrazil,
    #[serde(rename = "ro")]
    Romanian,
    #[serde(rename = "fi")]
    Finnish,
    #[serde(rename = "sv-SE")]
    Swedish,
    #[serde(rename = "vi")]
    Vietnamese,
    #[serde(rename = "tr")]
    Turkish,
    #[serde(rename = "cz")]
    Czech,
    #[serde(rename = "el")]
    Greek,
    #[serde(rename = "bg")]
    Bulgarian,
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "uk")]
    Ukrainian,
    #[serde(rename = "hi")]
    Hindi,
    #[serde(rename = "th")]
    Thai,
    #[serde(rename = "zh-CN")]
    ChineseChina,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "zh-TW")]
    ChineseTaiwan,
    #[serde(rename = "ko")]
    Korean,
}

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
