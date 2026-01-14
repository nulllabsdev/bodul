use std::fmt;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum RetailerCodeError {
    #[error("Invalid retailer code: {0}")]
    Invalid(String),
}

macro_rules! retailer_codes {
    (
        existing: {
            $( $existing_variant:ident => $existing_code:literal ),+ $(,)?
        },
        upcoming: {
            $( $upcoming_variant:ident => $upcoming_code:literal ),+ $(,)?
        },
        test: {
            $( $test_variant:ident => $test_code:literal ),+ $(,)?
        } $(,)?
    ) => {
        /// Retailer identifier
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum RetailerCode {
            // Production - Existing
            $( $existing_variant, )+
            // Production - Upcoming
            $( $upcoming_variant, )+
            // Test
            $( $test_variant, )+
        }

        const EXISTING_RETAILERS: &[RetailerCode] = &[
            $( RetailerCode::$existing_variant, )+
        ];

        const UPCOMING_RETAILERS: &[RetailerCode] = &[
            $( RetailerCode::$upcoming_variant, )+
        ];

        const TEST_RETAILERS_LIST: &[RetailerCode] = &[
            $( RetailerCode::$test_variant, )+
        ];

        impl RetailerCode {
            /// Get all active production retailers ready for data collection (31)
            pub const fn existing() -> &'static [RetailerCode] {
                EXISTING_RETAILERS
            }

            /// Get upcoming production retailers not yet ready (2)
            pub const fn upcoming() -> &'static [RetailerCode] {
                UPCOMING_RETAILERS
            }

            /// Get test retailers for development (3)
            pub const fn test_retailers() -> &'static [RetailerCode] {
                TEST_RETAILERS_LIST
            }

            /// Check if this is a test retailer
            pub const fn is_test(&self) -> bool {
                matches!(self, $( RetailerCode::$test_variant )|+)
            }

            /// Check if this is an upcoming retailer
            pub const fn is_upcoming(&self) -> bool {
                matches!(self, $( RetailerCode::$upcoming_variant )|+)
            }

            /// Convert to lowercase string representation
            pub const fn as_str(&self) -> &'static str {
                match self {
                    $( RetailerCode::$existing_variant => $existing_code, )+
                    $( RetailerCode::$upcoming_variant => $upcoming_code, )+
                    $( RetailerCode::$test_variant => $test_code, )+
                }
            }
        }

        impl fmt::Display for RetailerCode {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl FromStr for RetailerCode {
            type Err = RetailerCodeError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.to_ascii_lowercase().as_str() {
                    $( $existing_code => Ok(RetailerCode::$existing_variant), )+
                    $( $upcoming_code => Ok(RetailerCode::$upcoming_variant), )+
                    $( $test_code => Ok(RetailerCode::$test_variant), )+
                    _ => Err(RetailerCodeError::Invalid(s.to_string())),
                }
            }
        }
    };
}

retailer_codes! {
    existing: {
        Admhr => "admhr",
        Advenahr => "advenahr",
        Aloalohr => "aloalohr",
        Bazzarhr => "bazzarhr",
        Bigbanghr => "bigbanghr",
        Chipotekahr => "chipotekahr",
        Dmhr => "dmhr",
        Ekupihr => "ekupihr",
        Elipsohr => "elipsohr",
        Emmezetahr => "emmezetahr",
        Finderhr => "finderhr",
        Harveynormanhr => "harveynormanhr",
        Hgspothr => "hgspothr",
        Instarinformatikahr => "instarinformatikahr",
        Iqcentarhr => "iqcentarhr",
        Konzumhr => "konzumhr",
        Linkshr => "linkshr",
        Makromikrohr => "makromikrohr",
        Mallhr => "mallhr",
        Merkuryhr => "merkuryhr",
        Mihr => "mihr",
        Mikronishr => "mikronishr",
        Protishr => "protishr",
        Racunalahr => "racunalahr",
        Ronishr => "ronishr",
        Svijetmedijahr => "svijetmedijahr",
        Teammediahr => "teammediahr",
        Tehnomaghr => "tehnomaghr",
        Vacomhr => "vacomhr",
        Bigbangsi => "bigbangsi",
        Harveynormansi => "harveynormansi",
    },
    upcoming: {
        Istylehr => "istylehr",
        Storecomhr => "storecomhr",
    },
    test: {
        BoxClub => "boxclub",
        CheapyMart => "cheapymart",
        ShadyShop => "shadyshop",
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_existing_count() {
        assert_eq!(RetailerCode::existing().len(), 31);
    }

    #[test]
    fn test_upcoming_count() {
        assert_eq!(RetailerCode::upcoming().len(), 2);
    }

    #[test]
    fn test_test_retailers_count() {
        assert_eq!(RetailerCode::test_retailers().len(), 3);
    }

    #[test]
    fn test_upcoming_contains_correct_retailers() {
        let upcoming = RetailerCode::upcoming();
        assert!(upcoming.contains(&RetailerCode::Istylehr));
        assert!(upcoming.contains(&RetailerCode::Storecomhr));
    }

    #[test]
    fn test_test_retailers_contains_correct_retailers() {
        let test = RetailerCode::test_retailers();
        assert!(test.contains(&RetailerCode::BoxClub));
        assert!(test.contains(&RetailerCode::CheapyMart));
        assert!(test.contains(&RetailerCode::ShadyShop));
    }

    #[test]
    fn test_is_test() {
        assert!(RetailerCode::BoxClub.is_test());
        assert!(RetailerCode::CheapyMart.is_test());
        assert!(RetailerCode::ShadyShop.is_test());
        assert!(!RetailerCode::Admhr.is_test());
    }

    #[test]
    fn test_is_upcoming() {
        assert!(RetailerCode::Istylehr.is_upcoming());
        assert!(RetailerCode::Storecomhr.is_upcoming());
        assert!(!RetailerCode::Admhr.is_upcoming());
        assert!(!RetailerCode::BoxClub.is_upcoming());
    }

    #[test]
    fn test_display() {
        assert_eq!(RetailerCode::Admhr.to_string(), "admhr");
        assert_eq!(RetailerCode::Bigbangsi.to_string(), "bigbangsi");
        assert_eq!(RetailerCode::BoxClub.to_string(), "boxclub");
    }

    #[test]
    fn test_from_str_lowercase() {
        assert_eq!("admhr".parse::<RetailerCode>().unwrap(), RetailerCode::Admhr);
        assert_eq!(
            "bigbangsi".parse::<RetailerCode>().unwrap(),
            RetailerCode::Bigbangsi
        );
    }

    #[test]
    fn test_from_str_uppercase() {
        assert_eq!(
            "ADMHR".parse::<RetailerCode>().unwrap(),
            RetailerCode::Admhr
        );
        assert_eq!(
            "BIGBANGSI".parse::<RetailerCode>().unwrap(),
            RetailerCode::Bigbangsi
        );
    }

    #[test]
    fn test_from_str_mixed_case() {
        assert_eq!(
            "AdmHr".parse::<RetailerCode>().unwrap(),
            RetailerCode::Admhr
        );
    }

    #[test]
    fn test_from_str_invalid() {
        assert!("invalid".parse::<RetailerCode>().is_err());
        assert!("foo".parse::<RetailerCode>().is_err());
    }

    #[test]
    fn test_roundtrip_parse_display() {
        let codes = [
            RetailerCode::Admhr,
            RetailerCode::Bigbangsi,
            RetailerCode::BoxClub,
            RetailerCode::Istylehr,
        ];

        for code in &codes {
            let s = code.to_string();
            let parsed: RetailerCode = s.parse().unwrap();
            assert_eq!(parsed, *code);
        }
    }

}
