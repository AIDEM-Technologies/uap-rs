use super::*;

#[derive(Debug, Display, From)]
pub enum Error {
    Regex(regex::Error),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Matcher {
    #[serde(with = "serde_regex")]
    pub regex: regex::Regex,
    pub device_replacement: Option<String>,
    pub brand_replacement: Option<String>,
    pub model_replacement: Option<String>,
    pub device_replacement_has_group: bool,
    pub brand_replacement_has_group: bool,
    pub model_replacement_has_group: bool,
}

impl<'a> SubParser<'a> for Matcher {
    type Item = Device<'a>;

    fn try_parse(&self, text: &'a str) -> Option<Self::Item> {
        if !self.regex.is_match(text) {
            return None;
        }

        if let Some(captures) = self.regex.captures(text) {
            let family: Cow<'a, str> =
                if let Some(device_replacement) = &self.device_replacement {
                    replace_cow(
                        device_replacement,
                        self.device_replacement_has_group,
                        &captures,
                    )
                } else {
                    captures
                        .get(1)
                        .map(|x| x.as_str())
                        .and_then(none_if_empty)
                        .map(Cow::Borrowed)?
                };

            let brand: Option<Cow<'a, str>> = self
                .brand_replacement
                .as_ref()
                .map(|br| replace_cow(br, self.brand_replacement_has_group, &captures))
                .and_then(none_if_empty);

            let model: Option<Cow<'a, str>> =
                if let Some(model_replacement) = &self.model_replacement {
                    none_if_empty(replace_cow(
                        model_replacement,
                        self.model_replacement_has_group,
                        &captures,
                    ))
                } else {
                    captures
                        .get(1)
                        .map(|x| x.as_str())
                        .and_then(none_if_empty)
                        .map(Cow::Borrowed)
                };

            Some(Device {
                family,
                brand,
                model,
            })
        } else {
            None
        }
    }
}

impl Matcher {
    pub fn try_from(entry: DeviceParserEntry) -> Result<Matcher, Error> {
        let regex_with_flags = if entry.regex_flag.as_ref().map_or(true, String::is_empty)
        {
            entry.regex
        } else {
            format!("(?{}){}", entry.regex_flag.unwrap_or_default(), entry.regex)
        };
        let regex = regex::RegexBuilder::new(&clean_escapes(&regex_with_flags))
            .size_limit(20 * (1 << 20))
            .build();

        Ok(Matcher {
            regex: regex?,
            device_replacement_has_group: entry
                .device_replacement
                .as_ref()
                .map_or(false, |x| has_group(x.as_str())),
            device_replacement: entry.device_replacement,
            brand_replacement_has_group: entry
                .brand_replacement
                .as_ref()
                .map_or(false, |x| has_group(x.as_str())),
            brand_replacement: entry.brand_replacement,
            model_replacement_has_group: entry
                .model_replacement
                .as_ref()
                .map_or(false, |x| has_group(x.as_str())),
            model_replacement: entry.model_replacement,
        })
    }
}
