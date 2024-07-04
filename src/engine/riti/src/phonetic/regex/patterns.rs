pub(crate) struct Rule<'a> {
    pub(crate) matches: &'a [Match<'a>],
    pub(crate) replace: &'a str,
}

pub(crate) struct Pattern<'a> {
    pub(crate) find: &'a str,
    pub(crate) replace: &'a str,
    pub(crate) rules: &'a [Rule<'a>],
}

pub(crate) struct Match<'a> {
    pub(crate) type_: Type,
    pub(crate) scope: Scope,
    pub(crate) value: &'a str,
    pub(crate) negative: bool,
}

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum Type {
    Prefix,
    Suffix,
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub(crate) enum Scope {
    Vowel,
    Consonant,
    Punctuation,
    Exact,
}

pub(crate) const MAX_PATTERN_LEN: usize = 5;
pub(crate) const VOWELS: &str = "aeiou";
pub(crate) const CONSONANT: &str = "bcdfghjklmnpqrstvwxyz";
pub(crate) const IGNORE: &str = "|()[]{}^$*+?.~!@#%&-_='\";<>/\\,:`";

pub(crate) const PATTERNS: &[Pattern] = &[
    Pattern { find: "chchh", replace: "((চ্ছ)|((চ|ছ|([চছ]্?(হ|ঃ|(হ্‌?))))্?(((চ|ছ|([চছ]্?(হ|ঃ|(হ্‌?))))্?((হ|ঃ|(হ্‌?)))?)|([চছ]্?((হ|ঃ|(হ্‌?))্?(হ|ঃ|(হ্‌?))))|([চছ]্?(হ|ঃ|(হ্‌?))্?(হ|ঃ|(হ্‌?)))))|((চ|ছ|([চছ]্?(হ|ঃ|(হ্‌?))))্?(চ|ছ|([চছ]্?(হ|ঃ|(হ্‌?))))্?(হ|ঃ|(হ্‌?)))|([চছ]্?(হ|ঃ|(হ্‌?))্?[চছ]্?(হ|ঃ|(হ্‌?))্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "ngkkh", replace: "(((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?((ক্ষ)|((ক্?ক?)্?(হ|ঃ|(হ্‌?)))|(ক?্?(খ|(ক্ষ)|(ক্?(হ|ঃ|(হ্‌?)))))|(ক্?ক্?(হ|ঃ|(হ্‌?)))))|((((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?ক)|([নণঁঙঞং]্?(গ|(জ্ঞ))্?ক))্?(খ|(ক্ষ)|(ক্?(হ|ঃ|(হ্‌?)))))|((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?ক্?(খ|(ক্ষ)|(ক্?(হ|ঃ|(হ্‌?)))))|((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?(ক্?ক?)্?(হ|ঃ|(হ্‌?)))|([নণঁঙঞং]্?(গ|(জ্ঞ))্?ক্?ক্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "ngksh", replace: "(((((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?ক)|([নণঁঙঞং]্?(গ|(জ্ঞ))্?ক))্?(স|শ|ষ|([সশষ]্?(হ|ঃ|(হ্‌?)))))|((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?ক্?(স|শ|ষ|([সশষ]্?(হ|ঃ|(হ্‌?)))))|((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?((ক্?(স|শ|ষ|([সশষ]্?(হ|ঃ|(হ্‌?)))))|((ক্?[সশষ])্?(হ|ঃ|(হ্‌?)))|(ক্?[সশষ]্?(হ|ঃ|(হ্‌?)))))|((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?(ক্?[সশষ])্?(হ|ঃ|(হ্‌?)))|([নণঁঙঞং]্?(গ|(জ্ঞ))্?ক্?[সশষ]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "kkhm", replace: "((((ক্ষ)|((ক্?ক?)্?(হ|ঃ|(হ্‌?)))|(ক?্?(খ|(ক্ষ)|(ক্?(হ|ঃ|(হ্‌?)))))|(ক্?ক্?(হ|ঃ|(হ্‌?))))্?ম)|((ক্?ক?)্?((হ|ঃ|(হ্‌?))্?ম))|(ক্?ক্?(হ|ঃ|(হ্‌?))্?ম)|(ক্?(খ|(ক্ষ)|(ক্?(হ|ঃ|(হ্‌?))))্?ম))", rules: &[] },
    Pattern { find: "kkhn", replace: "((((ক্ষ)|((ক্?ক?)্?(হ|ঃ|(হ্‌?)))|(ক?্?(খ|(ক্ষ)|(ক্?(হ|ঃ|(হ্‌?)))))|(ক্?ক্?(হ|ঃ|(হ্‌?))))্?[নণঁঙঞং])|((ক্?ক?)্?((হ|ঃ|(হ্‌?))্?[নণঁঙঞং]))|(ক্?ক্?(হ|ঃ|(হ্‌?))্?[নণঁঙঞং])|(ক্?(খ|(ক্ষ)|(ক্?(হ|ঃ|(হ্‌?))))্?[নণঁঙঞং]))", rules: &[] },
    Pattern { find: "kshm", replace: "((((ক্?(স|শ|ষ|([সশষ]্?(হ|ঃ|(হ্‌?)))))|((ক্?[সশষ])্?(হ|ঃ|(হ্‌?)))|(ক্?[সশষ]্?(হ|ঃ|(হ্‌?))))্?ম)|(ক্?(((স|শ|ষ|([সশষ]্?(হ|ঃ|(হ্‌?))))্?ম)|([সশষ]্?((হ|ঃ|(হ্‌?))্?ম))|([সশষ]্?(হ|ঃ|(হ্‌?))্?ম)))|((ক্?[সশষ])্?((হ|ঃ|(হ্‌?))্?ম))|(ক্?(স|শ|ষ|([সশষ]্?(হ|ঃ|(হ্‌?))))্?ম)|(ক্?[সশষ]্?(হ|ঃ|(হ্‌?))্?ম))", rules: &[] },
    Pattern { find: "kshn", replace: "((((ক্?(স|শ|ষ|([সশষ]্?(হ|ঃ|(হ্‌?)))))|((ক্?[সশষ])্?(হ|ঃ|(হ্‌?)))|(ক্?[সশষ]্?(হ|ঃ|(হ্‌?))))্?[নণঁঙঞং])|(ক্?(((স|শ|ষ|([সশষ]্?(হ|ঃ|(হ্‌?))))্?[নণঁঙঞং])|([সশষ]্?((হ|ঃ|(হ্‌?))্?[নণঁঙঞং]))|([সশষ]্?(হ|ঃ|(হ্‌?))্?[নণঁঙঞং])))|((ক্?[সশষ])্?((হ|ঃ|(হ্‌?))্?[নণঁঙঞং]))|(ক্?(স|শ|ষ|([সশষ]্?(হ|ঃ|(হ্‌?))))্?[নণঁঙঞং])|(ক্?[সশষ]্?(হ|ঃ|(হ্‌?))্?[নণঁঙঞং]))", rules: &[] },
    Pattern { find: "ngch", replace: "(((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?(চ|ছ|([চছ]্?(হ|ঃ|(হ্‌?)))))|((((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?[চছ])|([নণঁঙঞং]্?(গ|(জ্ঞ))্?[চছ]))্?(হ|ঃ|(হ্‌?)))|([নণঁঙঞং]্?(গ|(জ্ঞ))্?[চছ]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "nggh", replace: "(((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?(ঘ|((গ|(জ্ঞ))্?(হ|ঃ|(হ্‌?)))))|([নণঁঙঞং]্?((জ্ঞ)|((গ|(জ্ঞ))্?((গ|(জ্ঞ)))?))্?(হ|ঃ|(হ্‌?)))|([নণঁঙঞং]্?((((জ্ঞ)|((গ|(জ্ঞ))্?((গ|(জ্ঞ)))?))্?(হ|ঃ|(হ্‌?)))|((গ|(জ্ঞ))্?(ঘ|((গ|(জ্ঞ))্?(হ|ঃ|(হ্‌?)))))|((গ|(জ্ঞ))্?(গ|(জ্ঞ))্?(হ|ঃ|(হ্‌?)))))|([নণঁঙঞং]্?(গ|(জ্ঞ))্?(গ|(জ্ঞ))্?(হ|ঃ|(হ্‌?)))|((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?(গ|(জ্ঞ))্?(হ|ঃ|(হ্‌?)))|(((ঙ্গ)|((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?(গ|(জ্ঞ)))|([নণঁঙঞং]্?(গ|(জ্ঞ))্?(গ|(জ্ঞ))))্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "ngjh", replace: "(((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?(ঝ|(([জয]|(জ়))্?(হ|ঃ|(হ্‌?)))))|((((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?([জয]|(জ়)))|([নণঁঙঞং]্?(গ|(জ্ঞ))্?([জয]|(জ়))))্?(হ|ঃ|(হ্‌?)))|([নণঁঙঞং]্?(গ|(জ্ঞ))্?([জয]|(জ়))্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "ngkh", replace: "(((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?(খ|(ক্ষ)|(ক্?(হ|ঃ|(হ্‌?)))))|((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?ক্?(হ|ঃ|(হ্‌?)))|([নণঁঙঞং]্?(গ|(জ্ঞ))্?ক্?(হ|ঃ|(হ্‌?)))|((((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?ক)|([নণঁঙঞং]্?(গ|(জ্ঞ))্?ক))্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "ngkx", replace: "(((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?((ক্ষ)|(ক্?((ক্স)|ষ))))|((((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?ক)|([নণঁঙঞং]্?(গ|(জ্ঞ))্?ক))্?((ক্স)|ষ))|([নণঁঙঞং]্?(গ|(জ্ঞ))্?((ক্ষ)|(ক্?((ক্স)|ষ))))|([নণঁঙঞং]্?(গ|(জ্ঞ))্?ক্?((ক্স)|ষ)))", rules: &[] },
    Pattern { find: "shsh", replace: "((((স|শ|ষ|([সশষ]্?(হ|ঃ|(হ্‌?)))))?্?(স|শ|ষ|([সশষ]্?(হ|ঃ|(হ্‌?)))))|([সশষ]্?(হ|ঃ|(হ্‌?))্?[সশষ]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "thth", replace: "((ত্থ)|(((থ|ঠ|([তটৎ]্?(হ|ঃ|(হ্‌?)))))?্?(থ|ঠ|([তটৎ]্?(হ|ঃ|(হ্‌?)))))|([তটৎ]্?(হ|ঃ|(হ্‌?))্?[তটৎ]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "bbh", replace: "((ব?্?(ভ|(ব্?(হ|ঃ|(হ্‌?)))))|((ব্?ব?)্?(হ|ঃ|(হ্‌?)))|(ব্?ব্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "bdh", replace: "((ব্?(ধ|ঢ|([দড]্?(হ|ঃ|(হ্‌?)))))|((ব্?[দড])্?(হ|ঃ|(হ্‌?)))|(ব্?[দড]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "bhl", replace: "(((ভ|(ব্?(হ|ঃ|(হ্‌?))))্?ল)|(ব্?((হ|ঃ|(হ্‌?))্?ল))|(ব্?(হ|ঃ|(হ্‌?))্?ল))", rules: &[] },
    Pattern { find: "cch", replace: "(([চছ]্?(চ|ছ|([চছ]্?(হ|ঃ|(হ্‌?)))))|([চছ]্?[চছ]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "chh", replace: "(((চ|ছ|([চছ]্?(হ|ঃ|(হ্‌?))))্?((হ|ঃ|(হ্‌?)))?)|([চছ]্?((হ|ঃ|(হ্‌?))্?(হ|ঃ|(হ্‌?))))|([চছ]্?(হ|ঃ|(হ্‌?))্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "cng", replace: "((চ্ঞ)|([চছ]্?(ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ)))))|([চছ]্?[নণঁঙঞং]্?(গ|(জ্ঞ)))|(([চছ]্?[নণঁঙঞং])্?(গ|(জ্ঞ))))", rules: &[] },
    Pattern { find: "dbh", replace: "(([দড]্?(ভ|(ব্?(হ|ঃ|(হ্‌?)))))|(([দড]্?ব)্?(হ|ঃ|(হ্‌?)))|([দড]্?ব্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "ddh", replace: "(([দড]?্?(ধ|ঢ|([দড]্?(হ|ঃ|(হ্‌?)))))|(([দড]্?[দড]?)্?(হ|ঃ|(হ্‌?)))|([দড]্?[দড]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "dgh", replace: "(([দড]্?(ঘ|((গ|(জ্ঞ))্?(হ|ঃ|(হ্‌?)))))|(([দড]্?(গ|(জ্ঞ)))্?(হ|ঃ|(হ্‌?)))|([দড]্?(গ|(জ্ঞ))্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "dhm", replace: "(((ধ|ঢ|([দড]্?(হ|ঃ|(হ্‌?))))্?ম)|([দড]্?((হ|ঃ|(হ্‌?))্?ম))|([দড]্?(হ|ঃ|(হ্‌?))্?ম))", rules: &[] },
    Pattern { find: "dhn", replace: "(((ধ|ঢ|([দড]্?(হ|ঃ|(হ্‌?))))্?[নণঁঙঞং])|([দড]্?((হ|ঃ|(হ্‌?))্?[নণঁঙঞং]))|([দড]্?(হ|ঃ|(হ্‌?))্?[নণঁঙঞং]))", rules: &[] },
    Pattern { find: "gdh", replace: "(((গ|(জ্ঞ))্?(ধ|ঢ|([দড]্?(হ|ঃ|(হ্‌?)))))|((গ|(জ্ঞ))্?[দড]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "ggh", replace: "((((জ্ঞ)|((গ|(জ্ঞ))্?((গ|(জ্ঞ)))?))্?(হ|ঃ|(হ্‌?)))|((গ|(জ্ঞ))্?(ঘ|((গ|(জ্ঞ))্?(হ|ঃ|(হ্‌?)))))|((গ|(জ্ঞ))্?(গ|(জ্ঞ))্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "ghn", replace: "(((ঘ|((গ|(জ্ঞ))্?(হ|ঃ|(হ্‌?))))্?[নণঁঙঞং])|((গ|(জ্ঞ))্?((হ|ঃ|(হ্‌?))্?[নণঁঙঞং]))|((গ|(জ্ঞ))্?(হ|ঃ|(হ্‌?))্?[নণঁঙঞং]))", rules: &[] },
    Pattern { find: "jjh", replace: "(((([জয]|(জ়)))?্?(ঝ|(([জয]|(জ়))্?(হ|ঃ|(হ্‌?)))))|(হ্য)|(((হ্য)|(([জয]|(জ়))্?(([জয]|(জ়)))?))্?(হ|ঃ|(হ্‌?)))|(([জয]|(জ়))্?([জয]|(জ়))্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "jng", replace: "((জ্ঞ)|(([জয]|(জ়))্?(ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ)))))|(([জয]|(জ়))্?[নণঁঙঞং]্?(গ|(জ্ঞ))))", rules: &[] },
    Pattern { find: "kkh", replace: "((ক্ষ)|((ক্?ক?)্?(হ|ঃ|(হ্‌?)))|(ক?্?(খ|(ক্ষ)|(ক্?(হ|ঃ|(হ্‌?)))))|(ক্?ক্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "ksh", replace: "((ক্?(স|শ|ষ|([সশষ]্?(হ|ঃ|(হ্‌?)))))|((ক্?[সশষ])্?(হ|ঃ|(হ্‌?)))|(ক্?[সশষ]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "kxm", replace: "((((ক্ষ)|(ক্?((ক্স)|ষ)))্?ম)|(ক্?(((ক্স)|ষ)্?ম))|(ক্?((ক্স)|ষ)্?ম))", rules: &[] },
    Pattern { find: "kxn", replace: "((((ক্ষ)|(ক্?((ক্স)|ষ)))্?[নণঁঙঞং])|(ক্?(((ক্স)|ষ)্?[নণঁঙঞং]))|(ক্?((ক্স)|ষ)্?[নণঁঙঞং]))", rules: &[] },
    Pattern { find: "lbh", replace: "((ল্?(ভ|(ব্?(হ|ঃ|(হ্‌?)))))|((ল্?ব)্?(হ|ঃ|(হ্‌?)))|(ল্?ব্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "ldh", replace: "((ল্?(ধ|ঢ|([দড]্?(হ|ঃ|(হ্‌?)))))|((ল্?[দড])্?(হ|ঃ|(হ্‌?)))|(ল্?[দড]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "lgh", replace: "((ল্?(ঘ|((গ|(জ্ঞ))্?(হ|ঃ|(হ্‌?)))))|((ল্?(গ|(জ্ঞ)))্?(হ|ঃ|(হ্‌?)))|(ল্?(গ|(জ্ঞ))্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "lkh", replace: "((ল্?(খ|(ক্ষ)|(ক্?(হ|ঃ|(হ্‌?)))))|((ল্?ক)্?(হ|ঃ|(হ্‌?)))|(ল্?ক্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "lph", replace: "((ল্?(ফ|(প্?(হ|ঃ|(হ্‌?)))))|((ল্?প)্?(হ|ঃ|(হ্‌?)))|(ল্?প্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "mbh", replace: "((ম্?(ভ|(ব্?(হ|ঃ|(হ্‌?)))))|((ম্?ব)্?(হ|ঃ|(হ্‌?)))|(ম্?ব্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "mph", replace: "((ম্?(ফ|(প্?(হ|ঃ|(হ্‌?)))))|((ম্?প)্?(হ|ঃ|(হ্‌?)))|(ম্?প্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "mth", replace: "((ম্?(থ|ঠ|([তটৎ]্?(হ|ঃ|(হ্‌?)))))|((ম্?[তটৎ])্?(হ|ঃ|(হ্‌?)))|(ম্?[তটৎ]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "nch", replace: "(([নণঁঙঞং]্?(চ|ছ|([চছ]্?(হ|ঃ|(হ্‌?)))))|(((ঞ্চ)|([নণঁঙঞং]্?[চছ]))্?(হ|ঃ|(হ্‌?)))|([নণঁঙঞং]্?[চছ]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "ndh", replace: "(([নণঁঙঞং]্?(ধ|ঢ|([দড]্?(হ|ঃ|(হ্‌?)))))|(([নণঁঙঞং]্?[দড])্?(হ|ঃ|(হ্‌?)))|([নণঁঙঞং]্?[দড]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "ngc", replace: "(((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?[চছ])|([নণঁঙঞং]্?(গ|(জ্ঞ))্?[চছ]))", rules: &[] },
    Pattern { find: "ngg", replace: "((ঙ্গ)|((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?(গ|(জ্ঞ)))|([নণঁঙঞং]্?(গ|(জ্ঞ))্?(গ|(জ্ঞ))))", rules: &[] },
    Pattern { find: "ngh", replace: "((ঙ্ঘ)|([নণঁঙঞং]্?(ঘ|((গ|(জ্ঞ))্?(হ|ঃ|(হ্‌?)))))|([নণঁঙঞং]্?(গ|(জ্ঞ))্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "ngj", replace: "(((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?([জয]|(জ়)))|([নণঁঙঞং]্?(গ|(জ্ঞ))্?([জয]|(জ়))))", rules: &[] },
    Pattern { find: "ngk", replace: "(((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?ক)|([নণঁঙঞং]্?(গ|(জ্ঞ))্?ক))", rules: &[] },
    Pattern { find: "ngm", replace: "(((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?ম)|([নণঁঙঞং]্?(গ|(জ্ঞ))্?ম))", rules: &[] },
    Pattern { find: "ngx", replace: "(((ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))্?((ক্স)|ষ))|([নণঁঙঞং]্?(গ|(জ্ঞ))্?((ক্স)|ষ)))", rules: &[] },
    Pattern { find: "njh", replace: "(([নণঁঙঞং]্?(ঝ|(([জয]|(জ়))্?(হ|ঃ|(হ্‌?)))))|(((ঞ্জ)|([নণঁঙঞং]্?([জয]|(জ়))))্?(হ|ঃ|(হ্‌?)))|([নণঁঙঞং]্?([জয]|(জ়))্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "nkh", replace: "(([নণঁঙঞং]্?(খ|(ক্ষ)|(ক্?(হ|ঃ|(হ্‌?)))))|(((ঙ্ক)|([নণঁঙঞং]্?ক))্?(হ|ঃ|(হ্‌?)))|([নণঁঙঞং]্?ক্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "nsh", replace: "(([নণঁঙঞং]্?(স|শ|ষ|([সশষ]্?(হ|ঃ|(হ্‌?)))))|([নণঁঙঞং]্?[সশষ]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "nth", replace: "(([নণঁঙঞং]্?(থ|ঠ|([তটৎ]্?(হ|ঃ|(হ্‌?)))))|(([নণঁঙঞং]্?[তটৎ])্?(হ|ঃ|(হ্‌?)))|([নণঁঙঞং]্?[তটৎ]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "phl", replace: "(((ফ|(প্?(হ|ঃ|(হ্‌?))))্?ল)|(প্?((হ|ঃ|(হ্‌?))্?ল))|(প্?(হ|ঃ|(হ্‌?))্?ল))", rules: &[] },
    Pattern { find: "rri", replace: "(ঋ|ৃ|(([রড়ঢ়]|(হ্র))([রড়ঢ়]|(হ্র))([ইঈিী]|(য়[িী]))))", rules: &[] },
    Pattern { find: "shm", replace: "(((স|শ|ষ|([সশষ]্?(হ|ঃ|(হ্‌?))))্?ম)|([সশষ]্?((হ|ঃ|(হ্‌?))্?ম))|([সশষ]্?(হ|ঃ|(হ্‌?))্?ম))", rules: &[] },
    Pattern { find: "shn", replace: "(((স|শ|ষ|([সশষ]্?(হ|ঃ|(হ্‌?))))্?[নণঁঙঞং])|([সশষ]্?((হ|ঃ|(হ্‌?))্?[নণঁঙঞং]))|([সশষ]্?(হ|ঃ|(হ্‌?))্?[নণঁঙঞং]))", rules: &[] },
    Pattern { find: "ssh", replace: "(([সশষ]?্?(স|শ|ষ|([সশষ]্?(হ|ঃ|(হ্‌?)))))|([সশষ]্?[সশষ]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "t``", replace: "ৎ", rules: &[] },
    Pattern { find: "tth", replace: "((([তটৎ]?(্?)(থ|ঠ|([তটৎ](্?)(হ|ঃ|(হ্‌?)))))|(([তটৎ](্?)[তটৎ]?)(্?)(হ|ঃ|(হ্‌?)))|([তটৎ](্?)[তটৎ](্?)(হ|ঃ|(হ্‌?)))))", rules: &[] },
    Pattern { find: "tth", replace: "(([তটৎ]?্?(থ|ঠ|([তটৎ]্?(হ|ঃ|(হ্‌?)))))|(([তটৎ]্?[তটৎ]?)্?(হ|ঃ|(হ্‌?)))|([তটৎ]্?[তটৎ]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "zzh", replace: "((হ্য)|((জ|য|(জ়)|([‌‍]?্য))্?(ঝ|(([জয]|(জ়))্?(হ|ঃ|(হ্‌?)))))|(((হ্য)|((জ|য|(জ়)|([‌‍]?্য))্?((জ|য|(জ়)|([‌‍]?্য)))?))্?(হ|ঃ|(হ্‌?)))|((জ|য|(জ়)|([‌‍]?্য))্?(জ|য|(জ়)|([‌‍]?্য))্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "aa", replace: "(আ|(য়া)|া|((([অএ]্যা?)|[আএ]|([‌‍]?(্য)?া)|(য়া))((([অএ]্যা?)|[আএ]|([‌‍]?(্য)?া)|(য়া)))?))", rules: &[] },
    Pattern { find: "ai", replace: "(ঐ|ৈ|(([অএ]্যা?)|[আএ]|([‌‍]?(্য)?া)|(য়া))([ইঈিী]|(য়[িী])))", rules: &[] },
    Pattern { find: "au", replace: "(ঔ|ৌ(([অএ]্যা?)|[আএ]|([‌‍]?(্য)?া)|(য়া))([উঊুূ]|(য়[ুূ])))", rules: &[] },
    Pattern { find: "az", replace: "((([অএ]্যা?)|[আএ]|([‌‍]?(্য)?া)|(য়া))((জ|য|(জ়)|([‌‍]?্য)))?)", rules: &[] },
    Pattern { find: "bb", replace: "(ব্?ব?)", rules: &[] },
    Pattern { find: "bd", replace: "(ব্?[দড])", rules: &[] },
    Pattern { find: "bh", replace: "(ভ|(ব্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "bv", replace: "(ব?্?ভ)", rules: &[] },
    Pattern { find: "cc", replace: "([চছ]্?[চছ]?)", rules: &[] },
    Pattern { find: "ch", replace: "(চ|ছ|([চছ]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "ck", replace: "(ক|([চছ]্?ক))", rules: &[] },
    Pattern { find: "cn", replace: "([চছ]্?[নণঁঙঞং])", rules: &[] },
    Pattern { find: "db", replace: "([দড]্?ব)", rules: &[] },
    Pattern { find: "dd", replace: "([দড]্?[দড]?)", rules: &[] },
    Pattern { find: "dg", replace: "([দড]্?(গ|(জ্ঞ)))", rules: &[] },
    Pattern { find: "dh", replace: "(ধ|ঢ|([দড]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "ee", replace: "(ই|ঈ|ি|ী|(য়েই)|(((এ্যা?)|[এে]|([‌‍]?(্য)া)|(য়ে))((এ্যা?)|[এে]|([‌‍]?(্য)া)|(য়ে))))", rules: &[] },
    Pattern { find: "ey", replace: "(এ|ই|ে|(েই)|(এই)|ঈ|ী|(((এ্যা?)|[এে]|([‌‍]?(্য)া)|(য়ে))(য়|(ইয়)|([‌‍]?্য))))", rules: &[] },
    Pattern { find: "ff", replace: "(ফ্?ফ?)", rules: &[] },
    Pattern { find: "gg", replace: "((জ্ঞ)|((গ|(জ্ঞ))্?((গ|(জ্ঞ)))?))", rules: &[] },
    Pattern { find: "gh", replace: "(ঘ|((গ|(জ্ঞ))্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "hh", replace: "((হ|ঃ|(হ্‌?))্?(হ|ঃ|(হ্‌?)))", rules: &[] },
    Pattern { find: "hl", replace: "((হ|ঃ|(হ্‌?))্?ল)", rules: &[] },
    Pattern { find: "hm", replace: "((হ|ঃ|(হ্‌?))্?ম)", rules: &[] },
    Pattern { find: "hn", replace: "((হ|ঃ|(হ্‌?))্?[নণঁঙঞং])", rules: &[] },
    Pattern { find: "ia", replace: "((ঞা)|(([ইঈিী]|(য়[িী]))(([অএ]্যা?)|[আএ]|([‌‍]?(্য)?া)|(য়া))))", rules: &[] },
    Pattern { find: "jh", replace: "(ঝ|(([জয]|(জ়))্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "jj", replace: "((হ্য)|(([জয]|(জ়))্?(([জয]|(জ়)))?))", rules: &[] },
    Pattern { find: "kh", replace: "(খ|(ক্ষ)|(ক্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "kk", replace: "(ক্?ক?)", rules: &[] },
    Pattern { find: "ks", replace: "(ক্?[সশষ])", rules: &[] },
    Pattern { find: "kx", replace: "((ক্ষ)|(ক্?((ক্স)|ষ)))", rules: &[] },
    Pattern { find: "lb", replace: "(ল্?ব)", rules: &[] },
    Pattern { find: "ld", replace: "(ল্?[দড])", rules: &[] },
    Pattern { find: "lg", replace: "(ল্?(গ|(জ্ঞ)))", rules: &[] },
    Pattern { find: "lk", replace: "(ল্?ক)", rules: &[] },
    Pattern { find: "ll", replace: "((হ্ল)|(ল?্?ল)|(ল্?ল))", rules: &[] },
    Pattern { find: "lp", replace: "(ল্?প)", rules: &[] },
    Pattern { find: "mb", replace: "(ম্?ব)", rules: &[] },
    Pattern { find: "mm", replace: "((হ্ম)|(ম্?ম?))", rules: &[] },
    Pattern { find: "mp", replace: "(ম্?প)", rules: &[] },
    Pattern { find: "mt", replace: "(ম্?[তটৎ])", rules: &[] },
    Pattern { find: "nc", replace: "((ঞ্চ)|([নণঁঙঞং]্?[চছ]))", rules: &[] },
    Pattern { find: "nd", replace: "([নণঁঙঞং]্?[দড])", rules: &[] },
    Pattern { find: "ng", replace: "(ঙ|ং|ঞ|(ঙ্গ)|([নণঁঙঞং]্?(গ|(জ্ঞ))))", rules: &[] },
    Pattern { find: "nj", replace: "((ঞ্জ)|([নণঁঙঞং]্?([জয]|(জ়))))", rules: &[] },
    Pattern { find: "nk", replace: "((ঙ্ক)|([নণঁঙঞং]্?ক))", rules: &[] },
    Pattern { find: "nn", replace: "((হ্ণ)|(হ্ন)|([নণঁঙঞং]্?[নণঁঙঞং]?))", rules: &[] },
    Pattern { find: "nt", replace: "([নণঁঙঞং]্?[তটৎ])", rules: &[] },
    Pattern { find: "oi", replace: "(ঐ|ৈ|(([ওোঅ]|(অ্য)|(য়ো?))?([ইঈিী]|(য়[িী]))))", rules: &[] },
    Pattern { find: "oo", replace: "((([উঊুূ]|(য়[ুূ])))|(([ওোঅ]|(অ্য)|(য়ো?))?([ওোঅ]|(অ্য)|(য়ো?))?))", rules: &[] },
    Pattern { find: "ou", replace: "(ঔ|ৌ|(([ওোঅ]|(অ্য)|(য়ো?))?([উঊুূ]|(য়[ুূ]))))", rules: &[] },
    Pattern { find: "ph", replace: "(ফ|(প্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "pp", replace: "(প্?প?)", rules: &[] },
    Pattern { find: "qq", replace: "(ক্?ক?)", rules: &[] },
    Pattern { find: "rh", replace: "((([রড়ঢ়]|(হ্র)))|(([রড়ঢ়]|(হ্র))্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "ri", replace: "(ঋ|ৃ|(হৃ)|(([রড়ঢ়]|(হ্র))([ইঈিী]|(য়[িী]))))", rules: &[] },
    Pattern { find: "sh", replace: "(স|শ|ষ|([সশষ]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "ss", replace: "([সশষ]্?[সশষ]?)", rules: &[] },
    Pattern { find: "th", replace: "(থ|ঠ|([তটৎ]্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "tt", replace: "([তটৎ]্?[তটৎ]?)", rules: &[] },
    Pattern { find: "uu", replace: "(ঊ|ূ|(([উঊুূ]|(য়[ুূ]))(([উঊুূ]|(য়[ুূ])))?))", rules: &[] },
    Pattern { find: "vv", replace: "(ভ্?ভ?)", rules: &[] },
    Pattern { find: "xm", replace: "(((ক্স)|ষ)্?ম)", rules: &[] },
    Pattern { find: "xn", replace: "(((ক্স)|ষ)্?[নণঁঙঞং])", rules: &[] },
    Pattern { find: "zh", replace: "(ঝ|(([জয]|(জ়))্?(হ|ঃ|(হ্‌?))))", rules: &[] },
    Pattern { find: "zz", replace: "((হ্য)|((জ|য|(জ়)|([‌‍]?্য))্?((জ|য|(জ়)|([‌‍]?্য)))?))", rules: &[] },
    Pattern { find: "0", replace: "(০|(0)|(শূন্য))", rules: &[] },
    Pattern { find: "1", replace: "(১|(1)|(এক))", rules: &[] },
    Pattern { find: "2", replace: "(২|(2)|(দুই))", rules: &[] },
    Pattern { find: "3", replace: "(৩|(3)|(তিন))", rules: &[] },
    Pattern { find: "4", replace: "(৪|(4)|(চার))", rules: &[] },
    Pattern { find: "5", replace: "(৫|(5)|(পাঁচ))", rules: &[] },
    Pattern { find: "6", replace: "((6)|৬|(ছয়))", rules: &[] },
    Pattern { find: "7", replace: "(৭|(7)|(সাত))", rules: &[] },
    Pattern { find: "8", replace: "(৮|(8)|(আট))", rules: &[] },
    Pattern { find: "9", replace: "(৯|(9)|(নয়))", rules: &[] },
    Pattern { find: "`", replace: "", rules: &[] },
    Pattern { find: "a", replace: "(([অএ]্যা?)|[আএ]|([‌‍]?(্য)?া)|(য়া))", rules: &[] },
    Pattern { find: "b", replace: "ব", rules: &[] },
    Pattern { find: "c", replace: "([চছ])", rules: &[] },
    Pattern { find: "d", replace: "([দড])", rules: &[] },
    Pattern { find: "e", replace: "((এ্যা?)|[এে]|([‌‍]?(্য)া)|(য়ে))", rules: &[] },
    Pattern { find: "f", replace: "ফ", rules: &[] },
    Pattern { find: "g", replace: "(গ|(জ্ঞ))", rules: &[] },
    Pattern { find: "h", replace: "(হ|ঃ|(হ্‌?))", rules: &[] },
    Pattern { find: "i", replace: "([ইঈিী]|(য়[িী]))", rules: &[] },
    Pattern { find: "j", replace: "([জয]|(জ়))", rules: &[] },
    Pattern { find: "k", replace: "ক", rules: &[] },
    Pattern { find: "l", replace: "ল", rules: &[] },
    Pattern { find: "m", replace: "ম", rules: &[] },
    Pattern { find: "n", replace: "([নণঁঙঞং])", rules: &[] },
    Pattern { find: "o", replace: "(([ওোঅ]|(অ্য)|(য়ো?))?)", rules: &[Rule {
        matches: &[Match {
            type_: Type::Prefix,
            scope: Scope::Punctuation,
            value: "",
            negative: false,
        }],
        replace: "([ওোঅ]|(অ্য)|(য়ো?))"
    }] },
    Pattern { find: "p", replace: "প", rules: &[] },
    Pattern { find: "q", replace: "ক", rules: &[] },
    Pattern { find: "r", replace: "([রড়ঢ়]|(হ্র))", rules: &[] },
    Pattern { find: "s", replace: "([সশষ])", rules: &[] },
    Pattern { find: "t", replace: "([তটৎ])", rules: &[] },
    Pattern { find: "u", replace: "([উঊুূ]|(য়[ুূ]))", rules: &[] },
    Pattern { find: "v", replace: "ভ", rules: &[] },
    Pattern { find: "w", replace: "(ও|(ওয়)|(্ব))", rules: &[] },
    Pattern { find: "x", replace: "((ক্স)|ষ)", rules: &[] },
    Pattern { find: "y", replace: "(য়|(ইয়)|([‌‍]?্য))", rules: &[] },
    Pattern { find: "z", replace: "(জ|য|(জ়)|([‌‍]?্য))", rules: &[] },
];
