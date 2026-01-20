# spotifyQL
A SQL like query language for your spotify statistics

### Examples
```SQL
SELECT COUNT(name) FROM PLAYLIST(pl1) WHERE artist == "Arctic Monkeys";
```
would could the number of arctic monkeys songs in the playlist pl1

```SQL
SELECT id, name FROM ALBUM(Whatever people say ...) WHERE popularity > 50 && release_date < 2014-01-01;
```

```SQL
SELECT COUNT(name) FROM PLAYLISTS;
```

## Attributes
the data available is the following

### Track Data
 - id: String
 - name: String
 - duration: Int
 - album_name: String
 - album_id: String
 - artists: List of String
 - added_at: String
 - popularity: Int

Track data is used when the data source is a specific playlist or saved album e.g. `PLAYLIST(pl1)`

### Playlist Data
 - id: String
 - name: String
 - tracks_api: String
 - track_count: Int

Playlist data is used when the data source is just `PLAYLISTS`

### Album Data
 - id: String
 - name: String
 - track_count: u64
 - popularity: u8
 - album_type: String
 - release_date: String
 - artists: List of String
 - saved_at: String

Album data is used when the data source is just `ALBUMS`

## Process
1. tokenise input
2. parse tokens into a struct
3. fetch necessary data
4. run parsed struct on fetched data
5. format and output

fetched data is cached in-memory so if the program is restarted all data needs to be refreshed, in-memory cached data has a TTL of 30 minutes.

## TODO
 - [x] fix api query returning invalid access token
 - [x] code verifier must be hashed incorrectly
 - [x] implement token refreshing
 - [ ] check for errors on api response
 - [x] gathering targets
 - [x] applying conditions
 - [ ] displaying data
 - [x] add support for IN keyword with arrays
 - [x] add support for >, <, >=, <= for ints and floats
 - [ ] add support for NOT IN condition
 - [ ] add support for using * as a display target
 - [ ] make a date struct and change all current dates stored in strings to use it
 - [ ] output to a file?
 - [ ] make into a tui app?