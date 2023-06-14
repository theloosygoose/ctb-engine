# Creating Matchups
This proved to be pretty difficult and there are a number of different ways that 
I couldve gone with this to create a "smarter" AI but this seems to work just fine.

### Uses
Creating matchups should be done everytime that there is a subsitution in the game and not every time down the court. 

###### In the future I would like to make it a *little* randomized and smarter players can get worse defenders on them
## Single Number Comparision

#### Equation

$$(1.07^{height} * HEIGHT_SMOOTHING_MATCHUP ) + (1.5 * ability) $$
---

`height` is exponential because I want players with taller heights to be pinned against one another

`1.5` is pulled from `weights.rs` 
```rust
pub const HEIGHT_MATCHUP:f32 = 1.07;
pub const HEIGHT_SMOOTHING_MATCHUP:f32 = 0.4;
pub const RATING_MATCHUP:f32 = 1.5;
```

### Final Code 
```rust
pub fn gen(offense_creation:&Vec<OffPreview>, on_ball_defense:&Vec<DefPreview>) -> Vec<Matchup> {
    let mut matchups: Vec<Matchup> = vec![];

    let mut off_abil: Vec<(PersonId, u16)> = vec![];
    let mut def_abil: Vec<(PersonId, u16)> = vec![];

    offense_creation.iter().for_each(|player|{
        let value = ( (HEIGHT_MATCHUP.powi(player.3 as i32) * HEIGHT_SMOOTHING_MATCHUP).round() as u16) + (RATING_MATCHUP * player.1).round() as u16;

        off_abil.push((player.0.clone(), value));
    });

    on_ball_defense.iter().for_each(|player|{
        let value = ( (HEIGHT_MATCHUP.powi(player.3 as i32) * HEIGHT_SMOOTHING_MATCHUP).round() as u16) + (RATING_MATCHUP * player.1).round() as u16;

        def_abil.push((player.0.clone(), value));
    });

    off_abil.sort_by_key(|k| k.1);
    def_abil.sort_by_key(|k| k.1);

    println!("{:#?}", &off_abil);
    println!("{:#?}", &def_abil);

    off_abil.iter().enumerate().for_each(|(i, off_player)|{
        matchups.push(Matchup(off_player.0.clone(), def_abil[i].0.clone()))
    });

    matchups
}
```

### Breakdown

Here we do the [equation](#equation) from above on both `offense_creation` and `on_ball_defense`.
```rust
offense_creation.iter().for_each(|player|{
    let value = ( (HEIGHT_MATCHUP.powi(player.3 as i32) * HEIGHT_SMOOTHING_MATCHUP).round() as u16) + (RATING_MATCHUP * player.1).round() as u16;

    off_abil.push((player.0.clone(), value));
});

on_ball_defense.iter().for_each(|player|{
    let value = ( (HEIGHT_MATCHUP.powi(player.3 as i32) * HEIGHT_SMOOTHING_MATCHUP).round() as u16) + (RATING_MATCHUP * player.1).round() as u16;

    def_abil.push((player.0.clone(), value));
});
```
I then sort the two `Vec<PersonId, u16>` here

```rust

off_abil.sort_by_key(|k| k.1);
def_abil.sort_by_key(|k| k.1);

```

Then loop through pushing each by their sorted index to `Vec<Matchup>` 
```rust

off_abil.iter().enumerate().for_each(|(i, off_player)|{
    matchups.push(Matchup(off_player.0.clone(), def_abil[i].0.clone()))
});

```
