# Creating Matchups
This proved to be pretty difficult and there are a number of different ways that 
I couldve gone with this to create a "smarter" AI but this seems to work just fine.

### Uses
Creating matchups should be done everytime that there is a subsitution in the game and not every time down the court. 

###### In the future I would like to make it a *little* randomized and smarter players can get worse defenders on them
## Single Number Comparision

#### Equation
<math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
  <mstyle displaystyle="true" scriptlevel="0">
    <mrow data-mjx-texclass="ORD">
      <mtable rowspacing=".5em" columnspacing="1em" displaystyle="true">
        <mtr>
          <mtd>
            <mfrac>
              <mrow>
                <mi>h</mi>
                <mi>e</mi>
                <mi>i</mi>
                <mi>g</mi>
                <mi>h</mi>
                <msup>
                  <mi>t</mi>
                  <mn>2</mn>
                </msup>
              </mrow>
              <mn>150</mn>
            </mfrac>
            <mo>+</mo>
            <mo stretchy="false">(</mo>
            <mn>1.5</mn>
            <mo>&#x2217;</mo>
            <mi>c</mi>
            <mi>r</mi>
            <mi>e</mi>
            <mi>a</mi>
            <mi>t</mi>
            <mi>i</mi>
            <mi>o</mi>
            <mi>n</mi>
            <mo stretchy="false">)</mo>
          </mtd>
        </mtr>
      </mtable>
    </mrow>
  </mstyle>
</math>

---

`height` is exponential because I want players with taller heights to be pinned against one another

`150` and `1.5` are pulled from `weights.rs` 
```rust
pub const HEIGHT_MATCHUP:u16 = 150;

pub const RATING_MATCHUP:f32 = 1.5;
```

### Final Code 
```rust
impl Matchup {
    pub fn gen(offense_creation:&Vec<OffPreview>, on_ball_defense:&Vec<DefPreview>) -> Vec<Matchup> {
        let mut matchups: Vec<Matchup> = vec![];

        let mut off_abil: Vec<(PersonId, u16)> = vec![];
        let mut def_abil: Vec<(PersonId, u16)> = vec![];

        offense_creation.iter().for_each(|player|{
            let value = ((player.3 * player.3)/HEIGHT_MATCHUP) + (RATING_MATCHUP * player.1).round() as u16;

            off_abil.push((player.0.clone(), value));
        });

        on_ball_defense.iter().for_each(|player|{
            let value = ((player.3 * player.3)/HEIGHT_MATCHUP) + (RATING_MATCHUP * player.1).round() as u16;

            def_abil.push((player.0.clone(), value));
        });

        off_abil.sort_by_key(|k| k.1);
        def_abil.sort_by_key(|k| k.1);

        off_abil.iter().enumerate().for_each(|(i, off_player)|{
            matchups.push(Matchup(off_player.0.clone(), def_abil[i].0.clone()))
        });

        matchups
    }
}
```


### Breakdown

Here we do the [equation](#equation) from above on both `offense_creation` and `on_ball_defense`.
```rust
offense_creation.iter().for_each(|player|{
    let value = ((player.3 * player.3)/HEIGHT_MATCHUP) + (RATING_MATCHUP * player.1).round() as u16;
    off_abil.push((player.0.clone(), value));
});

on_ball_defense.iter().for_each(|player|{
    let value = ((player.3 * player.3)/HEIGHT_MATCHUP) + (RATING_MATCHUP * player.1).round() as u16;
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
