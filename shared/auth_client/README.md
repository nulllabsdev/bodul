# Auth Client

Library for internal service authentication.

## Security Warning

**Development Only Tokens**

This library currently uses hardcoded tokens (see `src/model.rs`) for service identification.
These are for **development and early testing only**.
Do not use these in production environment without a proper secrets management strategy (see EDR-0006).
