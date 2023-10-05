# Vite Chat App Backend

## Next Steps
- [x] Remove room functionality, implement websockets like in ts proj
- [ ] Implement message read server sent notification.
- [x] Implement token expiration.
- [x] Implement update email, password and username.
- [x] Implement update profile picture.
- [ ] Implement session functionality using redis
- [ ] Implement logout functionality

## Sea orm
generate entities from the database
```
> sea-orm-cli generate entity -u postgresql://postgres:myblogdbpass48Y4N@db.muynfblecxqewykqdwwx.supabase.co:5432/postgres -o src/repository/entities
sea-orm-cli generate entity -u postgresql://postgres:postgres@localhost:5433/new_test -o src/repository/entities
```

## Building

To build: `cargo build`

To run:
* `dotenv -e .env -- cargo run`

Additional notes:
* install dotenv with npm

API Changes:
- GET /api/group/message/{group_id} [DEPRECATED] -> GET /api/message/group?groupId={group_id}
- GET /api/group [DEPRECATED] -> GET /api/contact/group
- GET /api/group/recent -> GET /api/contact/group/recent
- PUT /api/message/read?receiverUid=3 -> websocket
- PUT /api/group/read/{group_id} -> websocket
