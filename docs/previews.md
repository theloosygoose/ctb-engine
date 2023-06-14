# Previews

#### So what are previews?
In short previews are just a single summarized stat that will be used in a lot of logic and checks. 

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct OffPreview(pub PersonId, pub f32, pub OffVal );
```

This is the struct `OffPreview` and it is just a `tuple` that hold.

- The `PersonID`
- The `f32` value
- And the type for that value 

...and the type has a certain State 
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum OffVal {
    Initiator,
    Driving,
    OffBall,
    FloorSpacing,
}
```
and each of the `OffVal` has a custom set of weights and stats to pull from.  
