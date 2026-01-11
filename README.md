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
SELECT COUNT(name) FROM PLAYLIST;
```

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
 - [ ] gathering targets
 - [ ] applying conditions
 - [ ] displaying data
 - [ ] make a date struct and change all current dates stored in strings to use it
 - [ ] output to a file?
 - [ ] make into a tui app?