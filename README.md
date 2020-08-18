### Gifs are not trending anymore, but what about ascii gifs?

![](ascii-gif-example.gif)

- retrieve a gif using tenor and/or giphy apis
- transform the gif into an "ascii gif"
- display the gif on the command line 
- enjoy!

#### How to install
```bash
cargo install ascii-gif
```

#### Examples
```bash
# retrieve a gif using tenor random api and querying "pikachu"
asci-gif --query "pikachu" --tenor

# retrieve a gif using tenor random api and querying "pikachu"
asci-gif --query "pikachu" --giphy

# retrieve the gif with id 15452657 using tenor api
asci-gif --id "15452657" --tenor

# retrieve the gif with id U2nN0ridM4lXy using giphy api
asci-gif --id "U2nN0ridM4lXy" --giphy
```

#### Ascii encoding
The ascii encoding can be performed using 10 or 70 characters.
The default encoding uses 10 characters.
It is possible to use 70 characters encoding using the `--encoding` parameter
```bash
asci-gif --id "U2nN0ridM4lXy" --giphy --encoding 70
``` 

#### Still a "work in progress" project

---
todos
- enhance ascii transformation (choose better characters / enrich characters)
- <s>stop cropping gifs</s>
- <s>cli arguments enhancement</s>
- turn code into better rust
