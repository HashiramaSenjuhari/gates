# Sergy

#### file and content search utility including dynamic search

```rust
    // actual dir /app/[id]/[id]/[id]/[id]/b.html
    let billionaire = page!("app", "app/b/b/b/b/b.html"); // it searches inside app directory and find data
    println!("{}", billionaire.unwrap());
```
