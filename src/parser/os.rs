use super::*;

#[derive(Debug, Display, From)]
pub enum Error {
    Regex(regex::Error),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct Matcher {
    #[serde(with = "serde_regex")]
    pub regex: regex::Regex,
    pub os_replacement: Option<String>,
    pub os_v1_replacement: Option<String>,
    pub os_v2_replacement: Option<String>,
    pub os_v3_replacement: Option<String>,
    pub os_replacement_has_group: bool,
    pub os_v1_replacement_has_group: bool,
    pub os_v2_replacement_has_group: bool,
    pub os_v3_replacement_has_group: bool,
}


impl<'a> SubParser<'a> for Matcher {
    type Item = OS<'a>;

    fn try_parse(&self, text: &'a str) -> Option<Self::Item> {
        if !self.regex.is_match(text) {
            return None;
        }

        if let Some(captures) = self.regex.captures(text) {
            let family: Cow<'a, str> = if let Some(os_replacement) = &self.os_replacement
            {
                replace_cow(os_replacement, self.os_replacement_has_group, &captures)
            } else {
                captures
                    .get(1)
                    .map(|x| x.as_str())
                    .and_then(none_if_empty)
                    .map(Cow::Borrowed)?
            };

            let major: Option<Cow<'a, str>> =
                if let Some(os_v1_replacement) = &self.os_v1_replacement {
                    none_if_empty(replace_cow(
                        os_v1_replacement,
                        self.os_v1_replacement_has_group,
                        &captures,
                    ))
                } else {
                    captures
                        .get(2)
                        .map(|x| x.as_str())
                        .and_then(none_if_empty)
                        .map(Cow::Borrowed)
                };

            let minor: Option<Cow<'a, str>> =
                if let Some(os_v2_replacement) = &self.os_v2_replacement {
                    none_if_empty(replace_cow(
                        os_v2_replacement,
                        self.os_v2_replacement_has_group,
                        &captures,
                    ))
                } else {
                    captures
                        .get(3)
                        .map(|x| x.as_str())
                        .and_then(none_if_empty)
                        .map(Cow::Borrowed)
                };

            let patch: Option<Cow<'a, str>> =
                if let Some(os_v3_replacement) = &self.os_v3_replacement {
                    none_if_empty(replace_cow(
                        os_v3_replacement,
                        self.os_v3_replacement_has_group,
                        &captures,
                    ))
                } else {
                    captures
                        .get(4)
                        .map(|x| x.as_str())
                        .and_then(none_if_empty)
                        .map(Cow::Borrowed)
                };

            let patch_minor: Option<Cow<'a, str>> = captures
                .get(5)
                .map(|x| x.as_str())
                .and_then(none_if_empty)
                .map(Cow::Borrowed);

            Some(OS {
                family,
                major,
                minor,
                patch,
                patch_minor,
            })
        } else {
            None
        }
    }
}

impl Matcher {
    pub fn try_from(entry: OSParserEntry) -> Result<Matcher, Error> {
        let regex = regex::Regex::new(&clean_escapes(&entry.regex));

        Ok(Matcher {
            regex: regex?,
            os_replacement_has_group: entry
                .os_replacement
                .as_ref()
                .map_or(false, |x| has_group(x.as_str())),
            os_replacement: entry.os_replacement,
            os_v1_replacement_has_group: entry
                .os_v1_replacement
                .as_ref()
                .map_or(false, |x| has_group(x.as_str())),
            os_v1_replacement: entry.os_v1_replacement,
            os_v2_replacement_has_group: entry
                .os_v2_replacement
                .as_ref()
                .map_or(false, |x| has_group(x.as_str())),
            os_v2_replacement: entry.os_v2_replacement,
            os_v3_replacement_has_group: entry
                .os_v3_replacement
                .as_ref()
                .map_or(false, |x| has_group(x.as_str())),
            os_v3_replacement: entry.os_v3_replacement,
        })
    }
}
