#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum RetailerCode {
    MinisForumEu,
    MinisForumUs,
    MinisForumUk,
    MinisForumFr,
    MinisForumCa,
    MinisForumAu,
    MinisForumKr,
    MinisForumJp,
    MinisForumRu,
    MinisForumHk,

    // Xiaomi
    MiCom,

    // UGREEN
    UgreenCom,
    UgreenUs,
    UgreenCa,
    UgreenEu,
    UgreenDe,
    UgreenUk,
    UgreenFr,
    UgreenNl,
    UgreenJp,
    UgreenKr,
    UgreenIn,
    UgreenNas,
    UgreenNasCa,
    UgreenNasEu,
    UgreenNasDe,
    UgreenNasUk,
    UgreenNasFr,
    UgreenNasEs,
    UgreenNasIt,
    UgreenNasAu,
    UgreenNasJp,

    // Anker
    AnkerCom,
    AnkerJapanCom,
    AnkerKr,
    AnkerItalyCom,
    AnkerNordicsCom,
    AnkerUk,
    AnkerCa,
    AnkerEu,
    AnkerDe,
    AnkerFr,
    AnkerPl,
    AnkerAu,
    AnkerNz,
    AnkerMy,
    AnkerVn,
}

#[derive(Debug, PartialEq)]
pub struct RetailerCodeConversionError(String);

impl std::fmt::Display for RetailerCodeConversionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "unknown retailer code: {}", self.0)
    }
}

impl std::error::Error for RetailerCodeConversionError {}

impl TryFrom<&str> for RetailerCode {
    type Error = RetailerCodeConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "minisforumeu" => Ok(Self::MinisForumEu),
            "minisforumus" => Ok(Self::MinisForumUs),
            "minisforumuk" => Ok(Self::MinisForumUk),
            "minisforumfr" => Ok(Self::MinisForumFr),
            "minisforumca" => Ok(Self::MinisForumCa),
            "minisforumau" => Ok(Self::MinisForumAu),
            "minisforumkr" => Ok(Self::MinisForumKr),
            "minisforumjp" => Ok(Self::MinisForumJp),
            "minisforumru" => Ok(Self::MinisForumRu),
            "minisforumhk" => Ok(Self::MinisForumHk),

            "micom" => Ok(Self::MiCom),

            "ugreencom" => Ok(Self::UgreenCom),
            "ugreenus" => Ok(Self::UgreenUs),
            "ugreenca" => Ok(Self::UgreenCa),
            "ugreeneu" => Ok(Self::UgreenEu),
            "ugreende" => Ok(Self::UgreenDe),
            "ugreenuk" => Ok(Self::UgreenUk),
            "ugreenfr" => Ok(Self::UgreenFr),
            "ugreennl" => Ok(Self::UgreenNl),
            "ugreenjp" => Ok(Self::UgreenJp),
            "ugreenkr" => Ok(Self::UgreenKr),
            "ugreenin" => Ok(Self::UgreenIn),
            "ugreennas" => Ok(Self::UgreenNas),
            "ugreennasca" => Ok(Self::UgreenNasCa),
            "ugreennaseu" => Ok(Self::UgreenNasEu),
            "ugreennasde" => Ok(Self::UgreenNasDe),
            "ugreennasuk" => Ok(Self::UgreenNasUk),
            "ugreennasfr" => Ok(Self::UgreenNasFr),
            "ugreennases" => Ok(Self::UgreenNasEs),
            "ugreennasit" => Ok(Self::UgreenNasIt),
            "ugreennasau" => Ok(Self::UgreenNasAu),
            "ugreennasjp" => Ok(Self::UgreenNasJp),

            "ankercom" => Ok(Self::AnkerCom),
            "ankerjapancom" => Ok(Self::AnkerJapanCom),
            "ankerkr" => Ok(Self::AnkerKr),
            "ankeritalycom" => Ok(Self::AnkerItalyCom),
            "ankernordicscom" => Ok(Self::AnkerNordicsCom),
            "ankeruk" => Ok(Self::AnkerUk),
            "ankerca" => Ok(Self::AnkerCa),
            "ankereu" => Ok(Self::AnkerEu),
            "ankerde" => Ok(Self::AnkerDe),
            "ankerfr" => Ok(Self::AnkerFr),
            "ankerpl" => Ok(Self::AnkerPl),
            "ankerau" => Ok(Self::AnkerAu),
            "ankernz" => Ok(Self::AnkerNz),
            "ankermy" => Ok(Self::AnkerMy),
            "ankervn" => Ok(Self::AnkerVn),

            unknown => Err(RetailerCodeConversionError(unknown.to_string())),
        }
    }
}

impl TryFrom<String> for RetailerCode {
    type Error = RetailerCodeConversionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl From<RetailerCode> for String {
    fn from(retailer_code: RetailerCode) -> Self {
        format!("{retailer_code:?}").to_lowercase()
    }
}

impl RetailerCode {
    /// Every known retailer code.
    pub const ALL: [RetailerCode; 47] = [
        RetailerCode::AnkerCom,
        RetailerCode::AnkerJapanCom,
        RetailerCode::AnkerKr,
        RetailerCode::AnkerItalyCom,
        RetailerCode::AnkerNordicsCom,
        RetailerCode::AnkerUk,
        RetailerCode::AnkerCa,
        RetailerCode::AnkerEu,
        RetailerCode::AnkerDe,
        RetailerCode::AnkerFr,
        RetailerCode::AnkerPl,
        RetailerCode::AnkerAu,
        RetailerCode::AnkerNz,
        RetailerCode::AnkerMy,
        RetailerCode::AnkerVn,
        RetailerCode::MiCom,
        RetailerCode::MinisForumEu,
        RetailerCode::MinisForumUs,
        RetailerCode::MinisForumUk,
        RetailerCode::MinisForumFr,
        RetailerCode::MinisForumCa,
        RetailerCode::MinisForumAu,
        RetailerCode::MinisForumKr,
        RetailerCode::MinisForumJp,
        RetailerCode::MinisForumRu,
        RetailerCode::MinisForumHk,
        RetailerCode::UgreenCom,
        RetailerCode::UgreenUs,
        RetailerCode::UgreenCa,
        RetailerCode::UgreenEu,
        RetailerCode::UgreenDe,
        RetailerCode::UgreenUk,
        RetailerCode::UgreenFr,
        RetailerCode::UgreenNl,
        RetailerCode::UgreenJp,
        RetailerCode::UgreenKr,
        RetailerCode::UgreenIn,
        RetailerCode::UgreenNas,
        RetailerCode::UgreenNasCa,
        RetailerCode::UgreenNasEu,
        RetailerCode::UgreenNasDe,
        RetailerCode::UgreenNasUk,
        RetailerCode::UgreenNasFr,
        RetailerCode::UgreenNasEs,
        RetailerCode::UgreenNasIt,
        RetailerCode::UgreenNasAu,
        RetailerCode::UgreenNasJp,
    ];

    // DEPRECATED: use from_str() instead
    pub fn from_slug(slug: &str) -> Option<RetailerCode> {
        Self::try_from(slug).ok()
    }

    // DEPRECATED: use as_str() instead
    pub fn slug(self) -> String {
        self.into()
    }

    pub fn from_str(slug: &str) -> Result<RetailerCode, String> {
        Self::try_from(slug).map_err(|error| error.to_string())
    }

    pub fn as_str(self) -> String {
        self.into()
    }
}
#[cfg(test)]
mod tests {
    use super::{RetailerCode, RetailerCodeConversionError};

    #[test]
    fn resolves_code_from_slug() {
        assert_eq!(
            RetailerCode::from_slug("minisforumeu"),
            Some(RetailerCode::MinisForumEu)
        );
        assert_eq!(RetailerCode::from_slug("nope"), None);
    }

    #[test]
    fn converts_string_to_retailer_code() {
        let retailer_code = RetailerCode::try_from("minisforumeu".to_string()).unwrap();

        assert_eq!(retailer_code, RetailerCode::MinisForumEu);
    }

    #[test]
    fn rejects_unknown_retailer_code() {
        let error = RetailerCode::try_from("unknown").unwrap_err();

        assert_eq!(error, RetailerCodeConversionError("unknown".to_string()));
    }

    #[test]
    fn converts_retailer_code_to_string_with_try_into() {
        let retailer_code: String = RetailerCode::MinisForumEu.try_into().unwrap();

        assert_eq!(retailer_code, "minisforumeu");
    }
}
