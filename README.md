# spotifyQL
A SQL like query language for your spotify statistics

e.g.
```SQL
SELECT COUNT(name) FROM PLAYLIST("pl1") WHERE artist == "Arctic Monkeys";
```

would could the number of arctic monkeys songs in the playlist pl1

## Process
1. tokenise input
2. parse tokens into a struct
3. fetch necessary data
4. run parsed struct on fetched data
5. format and output

fetched data is cached in-memory so if the program is restarted all data needs to be refreshed, in-memory cached data has a TTL of 30 minutes.

## TODO
 - [ ] fix api query returning invalid access token
 - [ ] code verifier must be hashed incorrectly
 - [ ] implement token refreshing