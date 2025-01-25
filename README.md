### GitHub User Activity CLI App
Project inspiration taken from [roadmap.sh](https://roadmap.sh/projects/github-user-activity). Built with Rust 1.84.0. 

### Usage
```
cargo run -- <github_username>
```

### Example
```
cargo run -- kamranahmedse 
```
Output (truncated):
``` 
- pushed 1 commit to kamranahmedse/developer-roadmap
- pushed 1 commit to kamranahmedse/developer-roadmap
- starred BlackGlory/copycat
... 
- pushed 2 commits to kamranahmedse/developer-roadmap
- created an issue comment in kamranahmedse/driver.js
- pushed 1 commit to kamranahmedse/driver.js
- closed a pull request in kamranahmedse/driver.js
- pushed 1 commit to kamranahmedse/driver.js
```

### Crates used 
- `reqwest` for making HTTP requests to the GitHub API 
- `tokio` for async requests   
- `serde` and `serde_json` for deserializing JSON