# archaeopteryx 🦖 🐔

## RSA 4096 key generation tool

Statically linked compile and install example:
```
$ docker run -v $PWD:/volume --rm -t clux/muslrust:stable cargo build --release
    Updating crates.io index
  Downloaded openssl-macros v0.1.0
  Downloaded bitflags v1.3.2
  Downloaded pkg-config v0.3.26
  Downloaded syn v1.0.109
  Downloaded openssl v0.10.45
  Downloaded quote v1.0.25
  Downloaded time v0.1.45
  Downloaded unicode-ident v1.0.8
  Downloaded autocfg v1.1.0
  Downloaded cc v1.0.79
  Downloaded foreign-types-shared v0.1.1
  Downloaded cfg-if v1.0.0
  Downloaded num-integer v0.1.45
  Downloaded once_cell v1.17.1
  Downloaded num-traits v0.2.15
  Downloaded foreign-types v0.3.2
  Downloaded iana-time-zone v0.1.53
  Downloaded chrono v0.4.24
  Downloaded proc-macro2 v1.0.52
  Downloaded openssl-sys v0.9.80
  Downloaded libc v0.2.140
  Downloaded 21 crates (1.8 MB) in 2.45s
   Compiling archaeopteryx v0.1.0 (/volume)
    Finished release [optimized] target(s) in 3m 45s
$ mv target/x86_64-unknown-linux-musl/release/archaeopteryx /usr/local/bin/
```

#### time and system forensic data to associate with the generated key

The archaeopteryx is faster than the openssl CLI tool, even when adding a system sh call to collect artifacts.

If you don't want the artifacts, those lines can safely be commented out which furthers the speed.

The artifacts (audit strings) include:

- user that executed the program
- hostname and network addresses
- running processes
- hashes of /boot and of archaeopteryx in $PATH
- users logged into the system

The artifacts come first on a single line after a UTC timestamp. Then the generated public RSA key followed by the private RSA key in PKCS8 PEM.

The intended usage is to them pipe the output to your file encryption system, file, named pipe, or whatever.

```
$ archaeopteryx > d.archa
$ rage -e -p -a -o $(date +%Y%m%d%H%M%S%N).age d.archa && shred d.archa && rm d.archa
```

Or perhaps the user is going to use copy and paste to insert it into a key management system, etc, so they just let it print to the terminal and not save it locally.
