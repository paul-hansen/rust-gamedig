Every protocol has its own response type(s), below is a listing of the overlapping fields on these responses.

If a cell is blank it doesn't exist, otherwise it contains the type of that data in the current column's response type.
In the case that a field that performs the same function exists in the current column's response type that name is annotated in brackets.

# Response table

| Field            | Generic          | GameSpy(1)   | GameSpy(2)   | GameSpy(3)   | Minecraft(Java)   | Minecraft(Bedrock)   | Valve    | Quake    | Proprietary: FFOW     | Proprietary: TheShip   |
| :--------------- | ---------------- | ------------ | ------------ | ------------ | ----------------- | -------------------- | -------- | -------- | -------- | --------- |
| name             | `Option<String>` | `String`     | `String`  | `String`     |                   | `String`             | `String` | `String` | `String` | `String`  |
| description      | `Option<String>` |              |              |              | `String`          |                      |          |          | `String` |           |
| game             | `Option<String>` | `String` (game_type) |      | `String` (game_type) |   | `Option<GameMode>` (game_mode) | `String` | | `String` (game_mode) | `String` |
| game_version     | `Option<String>` | `String`     |              | `String`     | `String` (version_name) |                | `String` (version) | `String` (version) | `String` (version) | `String` (version) |
| map              | `Option<String>` | `String`     | `String`     | `String`     |                   | `Option<String>`     | `String` | `String` | `String` | `String`  |
| players_maxmimum | `u64`            | `usize`      | `usize`      | `usize`      | `u32`             | `u32`                | `u8`     | `u8`     | `u8`     | `u8` (max_players) |
| players_online   | `u64`            | `usize`      | `usize`      | `usize`      | `u32`             | `u32`                | `u8`     | `u8`     | `u8`     | `u8` (players) |
| players_bots     | `Option<u64>`    |              |              |              |                   |                      | `u8`     |          |          | `u8` (bots) |
| has_password     | `Option<bool>`   | `bool`       | `bool`       | `bool`       |                   |                      | `bool`   |          | `bool`   | `bool`    |
| map_title        |                  | `Option<String>` |          |          |                   |                      |          |          |          |           |
| admin_contact    |                  | `Option<String>` |          |          |                   |                      |          |          |          |           |
| admin_name       |                  | `Option<String>` |          |          |                   |                      |          |          |          |           |
| players_minimum  |                  | `Option<u8>` | `Option<u8>` | `Option<u8>` |                   |                      |          |          |          |           |
| players          |                  | `Vec<Player>` | `Vec<Player>` | `Vec<Player>` |                 |                      | `Option<Vec<ServerPlayer>>` | `Vec<P>` | | `Vec<TheShipPlayer>` (player_details) |
| tournament       |                  | `bool`       |              |`bool`       |                   |                      |          |          |          |           |
| unused_entries   |                  | `Hashmap<String, String>` | | `HashMap<String, String>` | |              | `Option<ExtraData>` (extra_data) | `HashMap<String, String>` | | |
| teams            |                  |              | `Vec<Team>`  | `Vec<Team>`  |                   |                      |          |          |          |           |
| version_protocol |                  |              |              |             | `i32`             | `String`             | `u8` (protocol) |   | `u8` (protocol) | `u8` (protocol) |
| players_sample   |                  |              |              |             | `Option<Vec<Player>>` |                  |          |          |          |           |
| favicon          |                  |              |              |             | `Option<String>`  |                      |          |          |          |           |
| previews_chat    |                  |              |              |             | `Option<bool>`    |                      |          |          |          |           |
| enforces_secure_chat   |            |              |              |             | `Option<bool>`    |                      |          |          |          |           |
| server_type      |                  |              |              |             | `Server`          | `Server`             | `Server` |          |          | `Server`  |
| edition          |                  |              |              |             |                   | `String`             |          |          |          |           |
| id               |                  |              |              |             |                   | `String`             |          |          |          |           |
| rules            |                  |              |              |             |                   |                      | `Option<HashMap<String,String>>` | | | `HashMap<String,String>` |
| folder           |                  |              |              |             |                   |                      | `String` |          |          |           |
| appid            |                  |              |              |             |                   |                      | `u32`    |          |          |           |
| environment_type |                  |              |              |             |                   |                      | `Environment` |     | `Environment` |      |
| vac_secured      |                  |              |              |             |                   |                      | `bool`   |          | `bool`   | `bool`    |
| the_ship         |                  |              |              |             |                   |                      | `Option<TheShip>` | |          |           |
| is_mod           |                  |              |              |             |                   |                      | `bool`   |          |          |           |
| mod_data         |                  |              |              |             |                   |                      | `Option<ModData>` | |          |           |
| active_mod       |                  |              |              |             |                   |                      |          |          | `String` |           |
| round            |                  |              |              |             |                   |                      |          |          | `u8`     |           |
| rounds_maximum   |                  |              |              |             |                   |                      |          |          | `u8`     |           |
| time_left        |                  |              |              |             |                   |                      |          |          | `u16`    |           |
| port             |                  |              |              |             |                   |                      |          |          |          | `Option<u16>` |
| steam_id         |                  |              |              |             |                   |                      |          |          |          | `Option<u64>` |
| tv_port          |                  |              |              |             |                   |                      |          |          |          | `Option<u16>` |
| tv_name          |                  |              |              |             |                   |                      |          |          |          | `Option<String>` |
| keywords         |                  |              |              |             |                   |                      |          |          |          | `Option<string>` |
| mode             |                  |              |              |             |                   |                      |          |          |          | `u8`      |
| witnesses        |                  |              |              |             |                   |                      |          |          |          | `u8`      |
| duration         |                  |              |              |             |                   |                      |          |          |          | `u8`      |