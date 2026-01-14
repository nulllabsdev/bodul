# PR-0001: Target Retailers for Launch Phase

| Field               | Value                                                                                           |
|---------------------|-------------------------------------------------------------------------------------------------|
| Status              | Draft                                                                                           |
| Owner               | @miro                                                                                           |
| Created             | 2026-01-14                                                                                      |
| Last updated        | 2026-01-14                                                                                      |
| Related goal        | [Launch Goal 1](../../roadmap.md)                                                               |
| Related engineering | [ER-0001: Retailer Code Library](../../technical/requirements/ER-0001-retailer-code-library.md) |

## Summary

This document defines the initial set of Croatian and Slovenian retailers targeted for catalog data collection during the launch phase.

## Context

Build the data foundation for a Croatian & Slovenian price comparison platform—starting with electronics retailers—so consumers can find products, compare prices, and spot fake "sales" through price history tracking.

### Phasing Strategy

| Priority | Category    | Rationale                                        |
|----------|-------------|--------------------------------------------------|
| 1        | Electronics | Primary focus, clear product categorization      |
| 2        | General     | Expand coverage after electronics are stabilized |
| 3        | Groceries   | Last due to high SKU churn and complexity        |

## Target Retailers

**Total: 33 retailers** (31 Croatia, 2 Slovenia)

### Croatia (HR)

| Code                | Name                 | Website                           | City           | Status   |
|---------------------|----------------------|-----------------------------------|----------------|----------|
| admhr               | ADM                  | https://www.adm.hr                | Zagreb         | ready    |
| advenahr            | Advena Klimatizacija | https://www.advena.hr             | Solin          | ready    |
| aloalohr            | ALO ALO              | https://aloalo.hr                 | Split          | ready    |
| bazzarhr            | BAZZAR               | https://bazzar.hr                 | Zagreb         | ready    |
| bigbanghr           | BIG BANG             | https://www.bigbang.hr            | Sveta Nedelja  | ready    |
| chipotekahr         | Chipoteka            | https://www.chipoteka.hr          | Sesvete        | ready    |
| dmhr                | dm-drogerie markt    | https://www.dm.hr                 | Zagreb         | ready    |
| ekupihr             | eKupi                | https://www.ekupi.hr              | Buzin          | ready    |
| elipsohr            | Elipso               | https://www.elipso.hr             | Donji Stupnik  | ready    |
| emmezetahr          | Emmezeta             | https://www.emmezeta.hr           | Donji Stupnik  | ready    |
| finderhr            | Finder               | https://www.finder.hr             | Zagreb         | ready    |
| harveynormanhr      | Harvey Norman        | https://www.harveynorman.hr       | Zagreb         | ready    |
| hgspothr            | HGSPOT               | https://www.hgspot.hr             | Zagreb         | ready    |
| instarinformatikahr | Instar Informatika   | https://www.instar-informatika.hr | Zagreb         | ready    |
| iqcentarhr          | IQ Centar            | https://iqcentar.hr               | Zagreb         | ready    |
| istylehr            | iSTYLE               | https://www.istyle.hr             | Zagreb         | upcoming |
| konzumhr            | Konzum               | https://www.konzum.hr             | Zagreb         | ready    |
| linkshr             | LINKS                | https://www.links.hr              | Sveta Nedelja  | ready    |
| makromikrohr        | Makromikro Grupa     | https://www.makromikrogrupa.hr    | Velika Gorica  | ready    |
| mallhr              | MALL.HR              | https://www.mall.hr               | Zagreb         | ready    |
| merkuryhr           | Merkury              | https://www.merkury.hr            | Donji Stupnik  | ready    |
| mihr                | MI Store / Xiaomi    | https://www.mi.hr                 | Osijek         | ready    |
| mikronishr          | Mikronis             | https://www.mikronis.hr           | Zagreb         | ready    |
| protishr            | Protis               | https://www.protis.hr             | Sisak          | ready    |
| racunalahr          | Racunala.hr          | https://www.racunala.hr           | Sveta Nedelja  | ready    |
| ronishr             | Ronis                | https://www.ronis.hr              | Cakovec        | ready    |
| storecomhr          | STORE                | https://store.com.hr              | Zagreb         | upcoming |
| svijetmedijahr      | Svijet Medija        | https://www.svijet-medija.hr      | Zagreb         | ready    |
| teammediahr         | Team Media           | https://team-media.hr             | Gornji Desinec | ready    |
| tehnomaghr          | Tehno Mag            | https://tehno-mag.hr              | Zagreb         | ready    |
| vacomhr             | Vacom                | https://vacom.hr                  | Daruvar        | ready    |

### Slovenia (SI)

| Code           | Name          | Website                     | City      | Status |
|----------------|---------------|-----------------------------|-----------|--------|
| bigbangsi      | Big Bang      | https://www.bigbang.si      | Ljubljana | ready  |
| harveynormansi | Harvey Norman | https://www.harveynorman.si | Ljubljana | ready  |

### Test Retailers

| Code       | Name       | Purpose                             | Status |
|------------|------------|-------------------------------------|--------|
| boxclub    | BoxClub    | Generic test retailer               | test   |
| cheapymart | CheapyMart | Price comparison testing            | test   |
| shadyshop  | ShadyShop  | Edge case / suspicious data testing | test   |

## Success Criteria

- [x] Defined retailer codes for all target retailers
