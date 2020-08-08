### Gifs are not trending anymore, but what about ascii gifs?

![Alt Text](https://github.com/visd0m/ascii-gif/blob/master/example_tenor.gif)

- retrieve a gif using tenor and/or giphy apis
- transform the gif into an "ascii gif"
- display the gif on the command line 
- enjoy!

#### Examples
```bash
# retrieve a gif using tenor random api and querying "pikachu"
cargo run -- --query "pikachu" --tenor

# retrieve a gif using tenor random api and querying "pikachu"
cargo run -- --query "pikachu" --giphy

# retrieve the gif with id 15452657 using tenor api
cargo run -- --id "15452657" --tenor

# retrieve the gif with id U2nN0ridM4lXy using giphy api
cargo run -- --id "U2nN0ridM4lXy" --giphy
```

#### Ascii encoding
The ascii can be performed using 10 or 69 characters.
The default encoding is 10.
It is possible to use 60 chars encoding using parameter --encoding
```bash
cargo run -- --id "U2nN0ridM4lXy" --giphy --encoding 69
``` 

#### Still a "work in progress" project

---
todos
- enhance ascii transformation (choose better characters / enrich characters)
- stop cropping gifs (mvp solution)
- cli arguments enhancement
- turn code into better rust
