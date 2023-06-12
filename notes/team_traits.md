# Team Traits

The `Team` trait is applied to `Vec<Person>` the uses of each function and how they work are defined here.

 > Note: For all `tuples` the `0` value is always going to be offense and the `1` value will always be defense  

Offense & Defense Previews
--

```rust 
fn off_previews(self) -> Vec<PlayerOffPreview> {
    let mut previews:Vec<PlayerOffPreview> = vec![];
    
    self.iter().for_each(|player| {
        previews.push(player.off_ability())
    });
    
    previews
}
```
---
Offensive and Defensive Previews are used when trying to generate a single stat for a player that can be sorted from high -> low or from low -> high.

This is helpful when just trying to get a basic view on how good someone is on offense. 

---

#### Future Fixes / Ideas
It should be noted that at this time you are only able to generate `off_ability()` which uses the off_abillity weights.

I think sometime in the future I can pull from different weights to get different stats. **For Example**:
```rust 
fn off_previews(self, OFF_BALL_WEIGHTS) -> Vec<PlayerOffPreview> {
    ...
    iter()...{
        player.off_ability(OFF_BALL_WEIGHTS)
    }
}
```

Get Player 
--

```rust 
fn get_player(&self, player: &PersonId) -> Person {
    let player_data = self.iter().find(|person_data| {
       &person_data.person_id == player
    });
    
    player_data.unwrap().clone()
}
```
---
This is a pretty simple function. Take in `self` and input a `&Person` and it outputs a cloned `Person` with all of their ratings

#### Future Fixes / Ideas
Nothing much...Seems to do the job 

Probably look at just returning a `Person` in memory with `&` rather than just using `.clone()`. Will probably increase performance a *tad*

Generate Defensive Matchups 
--
Fuction that takes in the ratings of both teams players that are on the court. The only thing that should be considered


