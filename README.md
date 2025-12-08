# spotifyQL
A SQL like query language for your spotify statistics

e.g.
```SQL
SELECT COUNT(name) FROM PLAYLIST("pl1") WHERE artist == "Arctic Monkeys";
```

would could the number of arctic monkeys songs in the playlist pl1