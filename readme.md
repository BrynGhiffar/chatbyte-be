# Vite Chat App Backend

## Next Steps
- [x] Remove room functionality, implement websockets like in ts proj

## Sea orm
generate entities from the database
```
> sea-orm-cli generate entity -u postgresql://postgres:myblogdbpass48Y4N@db.muynfblecxqewykqdwwx.supabase.co:5432/postgres -o src/entities
```

## Building

To build: `cargo build`

To run:
* `dotenv -e .env -- cargo run`

Additional notes:
* install dotenv with npm