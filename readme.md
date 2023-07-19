# Vite Chat App Backend

## Next Steps
- [x] Remove room functionality, implement websockets like in ts proj
- [ ] Implement message read server sent notification.
- [x] Implement token expiration.
- [ ] Implement update email, password and username.
- [ ] Implement update profile picture.
- [ ] Implement session functionality using redis
- [ ] Implement logout functionality

## Sea orm
generate entities from the database
```
> sea-orm-cli generate entity -u postgresql://postgres:myblogdbpass48Y4N@db.muynfblecxqewykqdwwx.supabase.co:5432/postgres -o src/entities
sea-orm-cli generate entity -u postgresql://postgres:postgres@localhost:5433/new_test -o src/entities
```

## Building

To build: `cargo build`

To run:
* `dotenv -e .env -- cargo run`

Additional notes:
* install dotenv with npm