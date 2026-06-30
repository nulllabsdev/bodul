# Coding conventions

## Readable return expressions

Keep return expressions short and readable. Assign non-trivial or multi-argument
construction to a clearly named local before returning it; do not inline that
construction into nested wrappers or collections.

For domain events, use an `evt` binding before wrapping the event in `MvpEvent`.

```rust
// correct
let evt = SitemapRetrieved::new(cmd.retrieval_id, cmd.retailer_code, documents.len());

Ok(vec![MvpEvent::SitemapRetrieved(evt)])

// incorrect
Ok(vec![MvpEvent::SitemapRetrieved(SitemapRetrieved::new(
    cmd.retrieval_id,
    cmd.retailer_code,
    documents.len(),
))])
```

## Command handler structure

Command handlers must separate intention from implementation. The `execute`
method reads as a sequence of named steps; each step delegates to a private
method that owns the details.

```rust
// correct
fn execute(&self, command: Foo) -> Result<Vec<Event>, CommandError> {
    let retrieval = self.load_retrieval(command.retrieval_id)?;
    let config = Self::get_config(retrieval.retailer_code)?;
    let entity = Self::build_entity(&retrieval, config)?;
    self.store_entity(&entity)?;
    self.mark_retrieval_done(command.retrieval_id)?;
    let event = Self::build_event(&entity);
    Ok(vec![AppEvent::Done(event)])
}

// incorrect — implementation mixed into execute
fn execute(&self, command: Foo) -> Result<Vec<Event>, CommandError> {
    let retrieval = self.retrieval_repo.load(command.retrieval_id).storage_err()?;
    let config = config_fn(retrieval.retailer_code).ok_or_else(|| ...)?;
    // ... many more lines inline
}
```

## Repository `store` methods

`store` methods accept a fully constructed record struct, not individual fields.
The caller is responsible for building the struct (including any
serialization/size computation) before calling `store`.

```rust
// correct
self.repo.store(NewProcessedSitemap { id, retrieval_id, document, ... })?;

// incorrect
self.repo.store(id, retrieval_id, &document, url_count)?;
```

Conflicts must always raise an error. Never use `ON CONFLICT ... DO UPDATE`
(upsert) in `store`; a duplicate should propagate as a `UniqueViolation`.

## Diesel: prefer `insert_into` over raw SQL

Use `diesel::insert_into` with an `Insertable` struct instead of
`diesel::sql_query` for inserts. With the `serde_json` feature enabled, `Jsonb`
columns accept `serde_json::Value` directly — no `::jsonb` cast needed.

```rust
// correct
#[derive(Insertable)]
#[diesel(table_name = my_table)]
struct NewRecord {
    id: Uuid,
    data: serde_json::Value,  // Jsonb column — works with serde_json feature
}
diesel::insert_into(my_table::table).values(record).execute(&mut *conn)?;

// incorrect
diesel::sql_query("INSERT INTO my_table ... VALUES ($1, $2::jsonb)")
    .bind::<SqlUuid, _>(id)
    .bind::<Text, _>(json_string)
    .execute(&mut *conn)?;
```

## SQL in Rust (Diesel `sql_query`)

SQL strings shorter than 120 characters must be written as a single line, not
split with `\` continuations.

```rust
// correct — fits on one line
diesel::sql_query("UPDATE sitemap_retrievals SET status = 'failed', error = $2 WHERE id = $1")

// correct — too long, use \ continuations
diesel::sql_query(
    "INSERT INTO raw_sitemap_documents \
     (id, retrieval_id, url, last_modified, body, body_size, fetched_at) \
     VALUES ($1, $2, $3, $4, $5, $6, $7)",
)
```
