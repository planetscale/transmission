### Transmission

#### Usage
To specify a database URL to write data to, create a .env file with the following format:
```
DATABASE_URL=mysql://[USERNAME]:[PASSWORD]@[PORT]:3306/customer
```

Then run `cargo install --path .` to build the executable.
 
After that, simply run `transmission` to write data to MySQL.