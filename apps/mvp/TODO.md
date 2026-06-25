# TODO




## upstreaming/features


## dev

## Planned Work

- [x] Set up Poem as the web server and run it via `apps/mvp/src/bin/server.rs`.
- [x] Add Docker support.
- [x] Add database support.
- [ ] Set up Mulac with commanding and eventing.
- [ ] Create `apps/mvp/src/bin/daily.rs` to send commands that request sitemap
  retrievals.
- [ ] Create `apps/mvp/src/bin/commanding.rs` to run up to 20 commands.
- [ ] Create `apps/mvp/src/bin/eventing.rs` to listen for up to 20 events.
- [ ] Model database storage for sitemaps.
- [ ] Model database storage for processed sitemaps.
- [ ] Model database storage for product offers.
- [ ] Figure out scheduling.

## Workflow

- [ ] Current implementation flow:
  `RequestSitemapRetrieval -> SitemapRetrieved -> ProcessSitemap -> SitemapProcessed -> GroupSitemapContent -> SitemapContentGrouped`.
- [ ] Next step after grouped content:
  launch `DownloadOfferPage` for all products in grouped content, then
  `OfferPageDownloaded -> ProcessOfferPage`.
- `DownloadOfferPage`, `OfferPageDownloaded`, and `ProcessOfferPage` are not
  part of the current implementation.

## Notes

Unchecked items are planning targets only. Do not implement them until explicitly
requested.
