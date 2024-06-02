# arkaive
configurable auto-archiver

# config
```toml
[[archives]]
name = ""
input = "" # path to directory to be archived
output = "" # path to where the archive gets spat out
```

# install
not for you :|

# TODO (in no particular order)
[ ] add (optional) encryption with [age](https://github.com/FiloSottile/age)
  - using this because it seems simple to use and i personally use it already
[ ] fix error handling to be more idiomatic (i hadn't read anything about it)
[ ] better CLI (currently none lol)
[ ] more config option
  - include, exclude files/dirs (with regex i think) etc.
