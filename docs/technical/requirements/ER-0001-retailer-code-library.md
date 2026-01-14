# ER-0001: Retailer Code Library

| Field           | Value                                                                               |
|-----------------|-------------------------------------------------------------------------------------|
| Status          | Accepted                                                                            |
| Owner           | @miro                                                                               |
| Created         | 2026-01-14                                                                          |
| Last updated    | 2026-01-14                                                                          |
| Related product | [PR-0001: Target Retailers](../../product/requirements/PR-0001-target-retailers.md) |

## Summary

Create a Cargo workspace in `shared/` with foundational libraries for retailer identification and type-safe retailer code handling.

## Workspace Structure

```
shared/
├── Cargo.toml           # Workspace manifest
├── elemental/           # Core domain types
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── elemental_infra/     # Infrastructure utilities
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

### Workspace Manifest

```toml
# shared/Cargo.toml
[workspace]
resolver = "2"
members = ["elemental", "elemental_infra"]
```

## elemental Library

Core domain types shared across the platform.

### RetailerCode Enum

Define a `RetailerCode` enum representing all supported retailers with bidirectional string conversion.

#### Requirements

1. **Enum variants** for each retailer code (PascalCase)
2. **String conversion** via macro:
   - `"admhr"` ↔ `RetailerCode::Admhr`
   - Case-insensitive parsing
3. **Display/FromStr** traits implemented
4. **Test retailers** for development and testing

#### Retailer Codes

**Production retailers** (from PR-0001):

| Code                | Enum Variant        | Country | Status   |
|---------------------|---------------------|---------|----------|
| admhr               | Admhr               | HR      |          |
| advenahr            | Advenahr            | HR      |          |
| aloalohr            | Aloalohr            | HR      |          |
| bazzarhr            | Bazzarhr            | HR      |          |
| bigbanghr           | Bigbanghr           | HR      |          |
| bigbangsi           | Bigbangsi           | SI      |          |
| chipotekahr         | Chipotekahr         | HR      |          |
| dmhr                | Dmhr                | HR      |          |
| ekupihr             | Ekupihr             | HR      |          |
| elipsohr            | Elipsohr            | HR      |          |
| emmezetahr          | Emmezetahr          | HR      |          |
| finderhr            | Finderhr            | HR      |          |
| harveynormanhr      | Harveynormanhr      | HR      |          |
| harveynormansi      | Harveynormansi      | SI      |          |
| hgspothr            | Hgspothr            | HR      |          |
| instarinformatikahr | Instarinformatikahr | HR      |          |
| iqcentarhr          | Iqcentarhr          | HR      |          |
| istylehr            | Istylehr            | HR      | upcoming |
| konzumhr            | Konzumhr            | HR      |          |
| linkshr             | Linkshr             | HR      |          |
| makromikrohr        | Makromikrohr        | HR      |          |
| mallhr              | Mallhr              | HR      |          |
| merkuryhr           | Merkuryhr           | HR      |          |
| mihr                | Mihr                | HR      |          |
| mikronishr          | Mikronishr          | HR      |          |
| protishr            | Protishr            | HR      |          |
| racunalahr          | Racunalahr          | HR      |          |
| ronishr             | Ronishr             | HR      |          |
| storecomhr          | Storecomhr          | HR      | upcoming |
| svijetmedijahr      | Svijetmedijahr      | HR      |          |
| teammediahr         | Teammediahr         | HR      |          |
| tehnomaghr          | Tehnomaghr          | HR      |          |
| vacomhr             | Vacomhr             | HR      |          |

**Test retailers** (for development/testing):

| Code       | Enum Variant | Purpose                             |
|------------|--------------|-------------------------------------|
| boxclub    | BoxClub      | Generic test retailer               |
| cheapymart | CheapyMart   | Price comparison testing            |
| shadyshop  | ShadyShop    | Edge case / suspicious data testing |

#### Macro Design

```rust
// Usage example
retailer_codes! {
    // Production
    Admhr => "admhr",
    Advenahr => "advenahr",
    // ... etc

    // Test
    BoxClub => "boxclub",
    CheapyMart => "cheapymart",
    ShadyShop => "shadyshop",
}
```

The macro should generate:
- `enum RetailerCode { ... }`
- `impl Display for RetailerCode`
- `impl FromStr for RetailerCode`

#### Helper Methods

| Method             | Returns                                                         | Count |
|--------------------|-----------------------------------------------------------------|-------|
| `existing()`       | Production retailers ready for data collection                  | 31    |
| `upcoming()`       | Production retailers not yet ready (istylehr, storecomhr)       | 2     |
| `test_retailers()` | Test retailers for development (BoxClub, CheapyMart, ShadyShop) | 3     |

Additional instance methods:
- `is_test(&self) -> bool`
- `is_upcoming(&self) -> bool`

## elemental_infra Library

Infrastructure utilities that depend on `elemental` types. Initial scope TBD based on needs.

## Success Criteria

- [ ] Cargo workspace created in `shared/`
- [ ] `elemental` library with `RetailerCode` enum
- [ ] `retailer_codes!` macro with bidirectional conversion
- [ ] All 33 production + 3 test retailer codes defined
- [ ] Helper methods: `existing()`, `upcoming()`, `test_retailers()`
- [ ] Unit tests for string conversion and helper methods
- [ ] `elemental_infra` library scaffolded
