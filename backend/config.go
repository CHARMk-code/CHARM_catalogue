package main

import (
  "github.com/BurntSushi/toml";
)

type Config struct {
  Creds Creds "toml:creds"
  Backend Backend "toml:backend"
  Database Database "toml:database"
}

type Creds struct {
  Password string
  Secret string
}

type Backend struct {
  Secret_key string
  Upload_folder string
  Static_folder string
}

type Database struct {
  User string
  Pass string
  Db string
  Server string
  Port string
}


func check(e error) {
  if e != nil {
    panic(e)
  }
}



func getConfig() Config {
  var cfg Config

  _, toml_err := toml.DecodeFile("./config.toml", &cfg)
  check(toml_err)
  
  return cfg
}

