use chrono::Utc;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Text};
use mvp::database::{DatabaseConfig, connect};

#[derive(QueryableByName)]
struct RawTotals {
    #[diesel(sql_type = BigInt)]
    retrieval_count: i64,
    #[diesel(sql_type = BigInt)]
    file_count: i64,
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
struct SizeTotals {
    #[diesel(sql_type = BigInt)]
    retrieval_count: i64,
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
struct RawRow {
    #[diesel(sql_type = Text)]
    retailer_code: String,
    #[diesel(sql_type = BigInt)]
    retrieval_count: i64,
    #[diesel(sql_type = BigInt)]
    min_files: i64,
    #[diesel(sql_type = BigInt)]
    max_files: i64,
    #[diesel(sql_type = BigInt)]
    min_size: i64,
    #[diesel(sql_type = BigInt)]
    avg_size: i64,
    #[diesel(sql_type = BigInt)]
    max_size: i64,
}

#[derive(QueryableByName)]
struct ProcessedRow {
    #[diesel(sql_type = Text)]
    retailer_code: String,
    #[diesel(sql_type = BigInt)]
    retrieval_count: i64,
    #[diesel(sql_type = BigInt)]
    min_size: i64,
    #[diesel(sql_type = BigInt)]
    avg_size: i64,
    #[diesel(sql_type = BigInt)]
    max_size: i64,
}

#[derive(QueryableByName)]
struct GroupedRow {
    #[diesel(sql_type = Text)]
    retailer_code: String,
    #[diesel(sql_type = BigInt)]
    retrieval_count: i64,
    #[diesel(sql_type = BigInt)]
    min_size: i64,
    #[diesel(sql_type = BigInt)]
    avg_size: i64,
    #[diesel(sql_type = BigInt)]
    max_size: i64,
    #[diesel(sql_type = BigInt)]
    avg_products: i64,
    #[diesel(sql_type = BigInt)]
    avg_catalogs: i64,
    #[diesel(sql_type = BigInt)]
    avg_content: i64,
    #[diesel(sql_type = BigInt)]
    avg_not_interested: i64,
    #[diesel(sql_type = BigInt)]
    avg_unknown: i64,
}

fn fmt_num(n: i64) -> String {
    let s = n.to_string();
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
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
    let config = DatabaseConfig::from_env();
    let pool = connect(&config).expect("DB pool");
    let mut conn = pool.get().expect("DB connection");

    let raw_totals = diesel::sql_query(
        "WITH per_retrieval AS ( \
            SELECT rsd.retrieval_id, \
                   COUNT(*)::bigint           AS file_count, \
                   SUM(rsd.body_size)::bigint AS total_size \
            FROM raw_sitemap_documents rsd \
            GROUP BY rsd.retrieval_id \
        ) \
        SELECT COUNT(*)::bigint               AS retrieval_count, \
               SUM(file_count)::bigint        AS file_count, \
               MIN(total_size)::bigint        AS min_size, \
               ROUND(AVG(total_size))::bigint AS avg_size, \
               MAX(total_size)::bigint        AS max_size, \
               SUM(total_size)::bigint        AS total_size \
        FROM per_retrieval",
    )
    .get_result::<RawTotals>(&mut conn)
    .expect("raw totals query");

    let processed_totals = diesel::sql_query(
        "SELECT COUNT(*)::bigint                  AS retrieval_count, \
                MIN(document_size)::bigint        AS min_size, \
                ROUND(AVG(document_size))::bigint AS avg_size, \
                MAX(document_size)::bigint        AS max_size, \
                SUM(document_size)::bigint        AS total_size \
         FROM processed_sitemaps",
    )
    .get_result::<SizeTotals>(&mut conn)
    .expect("processed totals query");

    let grouped_totals = diesel::sql_query(
        "SELECT COUNT(*)::bigint                 AS retrieval_count, \
                MIN(content_size)::bigint        AS min_size, \
                ROUND(AVG(content_size))::bigint AS avg_size, \
                MAX(content_size)::bigint        AS max_size, \
                SUM(content_size)::bigint        AS total_size \
         FROM grouped_sitemap_contents",
    )
    .get_result::<SizeTotals>(&mut conn)
    .expect("grouped totals query");

    let raw_rows = diesel::sql_query(
        "WITH per_retrieval AS ( \
            SELECT sr.retailer_code, \
                   rsd.retrieval_id, \
                   COUNT(*)::bigint           AS file_count, \
                   SUM(rsd.body_size)::bigint AS total_size \
            FROM raw_sitemap_documents rsd \
            JOIN sitemap_retrievals sr ON sr.id = rsd.retrieval_id \
            GROUP BY sr.retailer_code, rsd.retrieval_id \
        ) \
        SELECT retailer_code, \
               COUNT(*)::bigint                AS retrieval_count, \
               MIN(file_count)::bigint         AS min_files, \
               MAX(file_count)::bigint         AS max_files, \
               MIN(total_size)::bigint         AS min_size, \
               ROUND(AVG(total_size))::bigint  AS avg_size, \
               MAX(total_size)::bigint         AS max_size \
        FROM per_retrieval \
        GROUP BY retailer_code \
        ORDER BY retailer_code",
    )
    .load::<RawRow>(&mut conn)
    .expect("raw query");

    let processed_rows = diesel::sql_query(
        "SELECT retailer_code, \
                COUNT(*)::bigint                  AS retrieval_count, \
                MIN(document_size)::bigint        AS min_size, \
                ROUND(AVG(document_size))::bigint AS avg_size, \
                MAX(document_size)::bigint        AS max_size \
         FROM processed_sitemaps \
         GROUP BY retailer_code \
         ORDER BY retailer_code",
    )
    .load::<ProcessedRow>(&mut conn)
    .expect("processed query");

    let grouped_rows = diesel::sql_query(
        "SELECT retailer_code, \
                COUNT(*)::bigint                        AS retrieval_count, \
                MIN(content_size)::bigint               AS min_size, \
                ROUND(AVG(content_size))::bigint        AS avg_size, \
                MAX(content_size)::bigint               AS max_size, \
                ROUND(AVG(product_count))::bigint       AS avg_products, \
                ROUND(AVG(catalog_count))::bigint       AS avg_catalogs, \
                ROUND(AVG(content_count))::bigint       AS avg_content, \
                ROUND(AVG(not_interested_count))::bigint AS avg_not_interested, \
                ROUND(AVG(unknown_count))::bigint       AS avg_unknown \
         FROM grouped_sitemap_contents \
         GROUP BY retailer_code \
         ORDER BY retailer_code",
    )
    .load::<GroupedRow>(&mut conn)
    .expect("grouped query");

    let today = Utc::now().format("%Y-%m-%d").to_string();
    let mut md = String::new();

    md.push_str("# Sitemap Sizes Report\n\n");
    md.push_str(&format!("Generated on {today}.\n\n"));
    md.push_str(
        "For raw sitemaps all files in a retrieval are summed before computing min/avg/max. \
         Processed and grouped have one document per retrieval.\n\n",
    );

    // Totals
    md.push_str("## Totals\n\n");
    md.push_str("| Stage | Retrievals | Files retrieved | Min KB | Avg KB | Max KB | Sum MB |\n");
    md.push_str("| ----- | ---------: | --------------: | ------: | ------: | ------: | -----: |\n");
    md.push_str(&format!(
        "| Raw | {} | {} | {} | {} | {} | {} |\n",
        raw_totals.retrieval_count,
        raw_totals.file_count,
        fmt_kb(raw_totals.min_size),
        fmt_kb(raw_totals.avg_size),
        fmt_kb(raw_totals.max_size),
        fmt_mb(raw_totals.total_size),
    ));
    md.push_str(&format!(
        "| Processed | {} | — | {} | {} | {} | {} |\n",
        processed_totals.retrieval_count,
        fmt_kb(processed_totals.min_size),
        fmt_kb(processed_totals.avg_size),
        fmt_kb(processed_totals.max_size),
        fmt_mb(processed_totals.total_size),
    ));
    md.push_str(&format!(
        "| Grouped | {} | — | {} | {} | {} | {} |\n",
        grouped_totals.retrieval_count,
        fmt_kb(grouped_totals.min_size),
        fmt_kb(grouped_totals.avg_size),
        fmt_kb(grouped_totals.max_size),
        fmt_mb(grouped_totals.total_size),
    ));

    md.push('\n');

    // Raw per retailer
    md.push_str("## Raw Sitemaps\n\n");
    md.push_str("| Retailer | Retrievals | #Files | Min KB | Avg KB | Max KB |\n");
    md.push_str("| -------- | ---------: | --------------: | ------: | ------: | ------: |\n");
    for row in &raw_rows {
        let files = if row.min_files == row.max_files {
            row.min_files.to_string()
        } else {
            format!("{}–{}", row.min_files, row.max_files)
        };
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            row.retailer_code,
            row.retrieval_count,
            files,
            fmt_kb(row.min_size),
            fmt_kb(row.avg_size),
            fmt_kb(row.max_size),
        ));
    }

    md.push('\n');

    // Processed per retailer
    md.push_str("## Processed Sitemaps\n\n");
    md.push_str("| Retailer | Retrievals | Min KB | Avg KB | Max KB |\n");
    md.push_str("| -------- | ---------: | ------: | ------: | ------: |\n");
    for row in &processed_rows {
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            row.retailer_code,
            row.retrieval_count,
            fmt_kb(row.min_size),
            fmt_kb(row.avg_size),
            fmt_kb(row.max_size),
        ));
    }

    md.push('\n');

    // Grouped per retailer
    md.push_str("## Grouped Sitemap Contents\n\n");
    md.push_str("| Retailer | Retrievals | Min KB | Avg KB | Max KB | Products | Catalogs | Content | Skipped | Unknown |\n");
    md.push_str("| -------- | ---------: | ------: | ------: | ------: | -------: | -------: | ------: | ------: | ------: |\n");
    for row in &grouped_rows {
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            row.retailer_code,
            row.retrieval_count,
            fmt_kb(row.min_size),
            fmt_kb(row.avg_size),
            fmt_kb(row.max_size),
            fmt_num(row.avg_products),
            fmt_num(row.avg_catalogs),
            fmt_num(row.avg_content),
            fmt_num(row.avg_not_interested),
            fmt_num(row.avg_unknown),
        ));
    }

    std::fs::write("docs/sitemap-sizes-report.md", &md).expect("write report");
    println!("done: docs/sitemap-sizes-report.md");
}
