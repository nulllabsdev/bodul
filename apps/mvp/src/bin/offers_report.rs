use chrono::Utc;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Text};
use mvp::database::{DatabaseConfig, connect};

#[derive(QueryableByName)]
struct OfferTotals {
    #[diesel(sql_type = BigInt)]
    total: i64,
    #[diesel(sql_type = BigInt)]
    discovered: i64,
    #[diesel(sql_type = BigInt)]
    downloaded: i64,
    #[diesel(sql_type = BigInt)]
    failed: i64,
    #[diesel(sql_type = BigInt)]
    skipped: i64,
}

#[derive(QueryableByName)]
struct RawTotals {
    #[diesel(sql_type = BigInt)]
    raw_count: i64,
    #[diesel(sql_type = BigInt)]
    min_size: i64,
    #[diesel(sql_type = BigInt)]
    avg_size: i64,
    #[diesel(sql_type = BigInt)]
    max_size: i64,
    #[diesel(sql_type = BigInt)]
    total_size: i64,
}

#[derive(QueryableByName)]
struct OfferRow {
    #[diesel(sql_type = Text)]
    retailer_code: String,
    #[diesel(sql_type = BigInt)]
    total: i64,
    #[diesel(sql_type = BigInt)]
    discovered: i64,
    #[diesel(sql_type = BigInt)]
    downloaded: i64,
    #[diesel(sql_type = BigInt)]
    failed: i64,
    #[diesel(sql_type = BigInt)]
    skipped: i64,
}

#[derive(QueryableByName)]
struct RawRow {
    #[diesel(sql_type = Text)]
    retailer_code: String,
    #[diesel(sql_type = BigInt)]
    raw_count: i64,
    #[diesel(sql_type = BigInt)]
    min_size: i64,
    #[diesel(sql_type = BigInt)]
    avg_size: i64,
    #[diesel(sql_type = BigInt)]
    max_size: i64,
}

fn fmt_num(n: i64) -> String {
    let s = n.to_string();
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(*ch);
    }
    result
}

fn fmt_kb(n: i64) -> String {
    fmt_num(n / 1000)
}

fn fmt_mb(n: i64) -> String {
    fmt_num(n / (1000 * 1000))
}

fn main() {
    dotenvy::dotenv().ok();
    let _guard = mvp::logging::init();
    let config = DatabaseConfig::from_env();
    let pool = connect(&config).expect("DB pool");
    let mut conn = pool.get().expect("DB connection");

    let offer_totals = diesel::sql_query(
        "SELECT COUNT(*)::bigint                                          AS total, \
                COUNT(*) FILTER (WHERE status = 'discovered')::bigint     AS discovered, \
                COUNT(*) FILTER (WHERE status = 'downloaded')::bigint     AS downloaded, \
                COUNT(*) FILTER (WHERE status = 'failed')::bigint         AS failed, \
                COUNT(*) FILTER (WHERE status = 'skipped6hlimit')::bigint AS skipped \
         FROM offers",
    )
    .get_result::<OfferTotals>(&mut conn)
    .expect("offer totals query");

    let raw_totals = diesel::sql_query(
        "SELECT COUNT(*)::bigint                        AS raw_count, \
                COALESCE(MIN(body_size), 0)::bigint     AS min_size, \
                COALESCE(ROUND(AVG(body_size)), 0)::bigint AS avg_size, \
                COALESCE(MAX(body_size), 0)::bigint     AS max_size, \
                COALESCE(SUM(body_size), 0)::bigint     AS total_size \
         FROM raw_offers",
    )
    .get_result::<RawTotals>(&mut conn)
    .expect("raw offer totals query");

    let offer_rows = diesel::sql_query(
        "SELECT retailer_code, \
                COUNT(*)::bigint                                          AS total, \
                COUNT(*) FILTER (WHERE status = 'discovered')::bigint     AS discovered, \
                COUNT(*) FILTER (WHERE status = 'downloaded')::bigint     AS downloaded, \
                COUNT(*) FILTER (WHERE status = 'failed')::bigint         AS failed, \
                COUNT(*) FILTER (WHERE status = 'skipped6hlimit')::bigint AS skipped \
         FROM offers \
         GROUP BY retailer_code \
         ORDER BY retailer_code",
    )
    .load::<OfferRow>(&mut conn)
    .expect("offers per retailer query");

    let raw_rows = diesel::sql_query(
        "SELECT o.retailer_code, \
                COUNT(*)::bigint                           AS raw_count, \
                COALESCE(MIN(ro.body_size), 0)::bigint     AS min_size, \
                COALESCE(ROUND(AVG(ro.body_size)), 0)::bigint AS avg_size, \
                COALESCE(MAX(ro.body_size), 0)::bigint     AS max_size \
         FROM raw_offers ro \
         JOIN offers o ON o.id = ro.offer_id \
         GROUP BY o.retailer_code \
         ORDER BY o.retailer_code",
    )
    .load::<RawRow>(&mut conn)
    .expect("raw offers per retailer query");

    let today = Utc::now().format("%Y-%m-%d").to_string();
    let mut md = String::new();

    md.push_str("# Offers Report\n\n");
    md.push_str(&format!("Generated on {today}.\n\n"));
    md.push_str(
        "Offers are the product pages discovered from grouped sitemap content. \
         Raw offers hold the downloaded page bodies; sizes are `body_size` in bytes.\n\n",
    );

    // Totals
    md.push_str("## Totals\n\n");
    md.push_str(
        "| Offers | Discovered | Downloaded | Failed | Skipped | Raw offers | Min KB | Avg KB | Max KB | Sum MB |\n",
    );
    md.push_str(
        "| -----: | ---------: | ---------: | -----: | ------: | ---------: | -----: | -----: | -----: | -----: |\n",
    );
    md.push_str(&format!(
        "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
        fmt_num(offer_totals.total),
        fmt_num(offer_totals.discovered),
        fmt_num(offer_totals.downloaded),
        fmt_num(offer_totals.failed),
        fmt_num(offer_totals.skipped),
        fmt_num(raw_totals.raw_count),
        fmt_kb(raw_totals.min_size),
        fmt_kb(raw_totals.avg_size),
        fmt_kb(raw_totals.max_size),
        fmt_mb(raw_totals.total_size),
    ));

    md.push('\n');

    // Offers per retailer
    md.push_str("## Offers per Retailer\n\n");
    md.push_str("| Retailer | Offers | Discovered | Downloaded | Failed | Skipped |\n");
    md.push_str("| -------- | -----: | ---------: | ---------: | -----: | ------: |\n");
    for row in &offer_rows {
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            row.retailer_code,
            fmt_num(row.total),
            fmt_num(row.discovered),
            fmt_num(row.downloaded),
            fmt_num(row.failed),
            fmt_num(row.skipped),
        ));
    }

    md.push('\n');

    // Raw offer sizes per retailer
    md.push_str("## Raw Offer Sizes per Retailer\n\n");
    md.push_str("| Retailer | Raw offers | Min KB | Avg KB | Max KB |\n");
    md.push_str("| -------- | ---------: | -----: | -----: | -----: |\n");
    for row in &raw_rows {
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            row.retailer_code,
            fmt_num(row.raw_count),
            fmt_kb(row.min_size),
            fmt_kb(row.avg_size),
            fmt_kb(row.max_size),
        ));
    }

    std::fs::write("docs/offers-report.md", &md).expect("write report");
    println!("done: docs/offers-report.md");
}
