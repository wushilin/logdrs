# logdrs
Log swallower, rotater and more

# Getting binary
1. Download source code
2. Run cargo build --release
3. Check binary in target/release folder. It should be called `logd`.


# Using it
```sh
$ run-my-program | logd --out app.log --size 100M --keep 20
```

Receive logs from run-my-program, write to app.log and rotate every 100 MiB (pre-write checked)

# Adding dates to logs
If your log does not print date time, you can use `-dated` to add a timestamp for eachline for you automatically
```sh
$ run-my-program | logd -o app.log -s 100M -k 20 -d
```

# Using Windows CRLF as new line
```bash
logd .... -crlf
```

# Getting help
```bash
$ logd --help
Usage: logd [OPTIONS] --out <OUT>

Options:
  -o, --out <OUT>    
  -s, --size <SIZE>  [default: 100M]
  -k, --keep <KEEP>  [default: 20]
  -d, --dated        
      --crlf         
  -h, --help         Print help
```
