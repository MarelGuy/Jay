loops, create a keyword to address when to check the condition, example:

```rust
while i>5 {
    i+=1
    control_condition,
}
```
this might be a good idea for certain applications where you would need an infinite loop with an if to check a condition the would be modified after the condition check.