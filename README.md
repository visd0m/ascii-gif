### Gifs are not trending anymore, but what about ascii gifs?

- retrieve a gif using tenor and/or giphy apis
- transform the gif into an "ascii gif"
- display the gif on the command line 
- enjoy!

#### Examples
```bash
# retrieve a gif using tenor random api querying "pikachu" 
cargo run "pikachu" --tenor
```
![Alt Text](https://github.com/visd0m/ascii-gif/blob/master/example_tenor.gif)

```bash
# retrieve a gif using giphy random api querying "pikachu" 
cargo run "pikachu" --giphy
```
![Alt Text](https://github.com/visd0m/ascii-gif/blob/master/example_homer_giphy.gif)

#### Still a "work in progress" project

---
todos
- enhance ascii transformation (choose better characters / enrich characters)
- stop cropping gifs (mvp solution)
- cli arguments enhancement
- turn code into better rust
