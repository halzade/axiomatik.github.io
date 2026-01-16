
`cargo install cargo-watch`

### Start application in dev mod

`cargo watch -x run`

### Create Editor User
`cargo run -- create-user lukas dev`

### Delete User
`cargo run -- delete-user lukas`

### Print from Database
`cargo run -- print-from-db "select * from user"`

### other commands

`ps aux | grep axiomatik`


### Notes

     *  in devel,
     *  changing files will cause the application to restart, because of cargo watch
