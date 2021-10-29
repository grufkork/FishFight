# Juice and Feel
**Juice**, or **game feel**, is all about giving feedback to confirm a player's actions and simply making pushing a button more satisfying. For games where flow and feeling matter, juice is the thing that makes the player understand your game without reading dials and understanding numbers. Juice gives a game life and soul that makes the player feel like a part of the world. Juice is what makes a game feel fun without actually making any mechanical difference. And juice might just be what makes or breaks your game.

Whenever you do something in real life you get a wide range of sensory input in return indicating that what you tried to do actually happened. That response grounds you and makes you realise the reality and weight of things. As you might guess, replicating that kind of feedback on a flatscreen is hard. To get anywhere near reality you need hyperreal effects and nerve-blasting feedback to fill in for all the missing senses, all the while walking the fine line of overloading your player and making a dry, unresponsive world.

Of course, not all games need equal amounts of juice. Some games are played mostly in your head, and therefore all that visual feedback is not needed. Civilization has little more than flavour animations when a unit attacks, a town is built or a nuke blows up. (Actually the last one is pretty nice, but the real juice is your enemy’s tears) These are rather abstract both mechanically and visually, rarely pertaining to situations and perspectives we encounter in our everyday life. These games would still work with only coloured circles and lines, because the strategy and the story exists in the player’s head. These games are *planning* games, where the payoff is to after thousands of years of careful strategising you triumph over your enemy. 

On the other side of the spectrum are *execution* games test your reflexes and motor skills, and are played visually with game continually telling the player what is going on. Here, the payoff is landing a single-frame-margin combo and watching your enemy explode into a glorious cloud of gibs. The juice that in a planning game exists in the player’s head must now be given by the game. 

For realistic first-person games, juicing is difficult and labour-intensive in practice but easier in concept, as you can model the feedback off reality to align with the player’s expectations. But a side-scrolling shooter like Fish Fight is more difficult in theory. I’d doubt you have a clear picture of how stylised two-dimensional fish running and shooting each other would look. For something so far detached from reality, you must work a lot in the same way as with symbol drawing: finding effects that represent their real-life counterparts without actually looking like them. 

The worst part about working with game feel is that it is only ever noticed if it is bad. Good game feel is only felt, never seen. If the juice is so grandiose that the player starts thinking of it, it will wear them out and possibly even make them sick if too much stuff is moving onscreen. But if there is not enough of it, the game will feel dry and lifeless. The juice is there to constantly trigger and must therefore be *just* right. 

Now, for some ways to actually squeeze some juice out of your games!

## Screen Shake
Screen shake is an immensely powerful tool for giving heavy, visceral feedback that disorients the player as if their whole body was actually rumbling. When the entire scene shakes the player *feels* it, usually without thinking of it. Regular flatscreen games are only connected to two of the players senses, vision and hearing, and that is all you have to work with. The ears can be shaken about with some explosion noises, but to equally rustle the player’s eyes you need screen shake. 

Screen shake moves the player’s viewpoint around, translationally and/or rotationally, to make it seem like the world is rumbling. Adding the right amount of shake is an art; it is crucial to not overdo screen shake as it can hamper readability and even make sensitive players motion sick, but at the same time it must be significant enough to be felt.   

### Screen shake in practice in Fish Fight

Screen shake is a lot about feel, and as such requires a lot of dialling in. In fish fight there are three kinds of screen shake: noise (Perlin) translational, sinusoidal translational and rotational. Translational noise moves the world up- and sideways, while the rotational noise rotates the world around the screen midpoint. To that, you can also make the screen shake only in a certain axis to give it directionality. Omnidirectional shaking is the most common and can be used for pretty much everything. It fits explosions really well, but won't feel out of place for anything else either. The directional type on the other hand is more specialised and can for example be used vertically for jump impacts or horizontally for guns firing sideways. This further enhances feedback and narrows its meaning down, increasing clarity and readability.

To those types there’s three parameters: magnitude, frequency and length. Magnitude affects how far the screen moves when shaking (the amplitude of the oscillation) while frequency controls how ”many” shakes happen per second. The length decides for how long the shake will last for, and the magnitude decreases over this time to zero.

By mixing and matching different values for these parameters different effects can be made. Here are a few examples:
- Gunshot: a short omnidirectional shake with low amplitude and high frequency. This gives a small kick that doesn’t move the screen too much, just enough to confirm the firing.
Fish Fight supports setting multipliers for X/Y movement, so a 1 for X and a smaller number for Y makes the player feel the direction of the action.
- Explosion: a medium-length omnidirectional shake with decent amplitude and lower frequency. This makes a kind of rumble that seriously jostles the screen. The bigger the explosive the lower the frequency!
- Jump impact: a directional sinusoidal shake makes the world bounce like a spring. 

To calculate the final scene position, the current position of every active shake is added together. That value is then capped using a logarithmic function so that if many explosives go off at once, the scene does not fly off-screen. The second logarithm (log<sub>2</sub>x) is almost linear from 0-1, and then flattens.

## Particles
Particles like those in games rarely occur in real life, as most of us go through our daily routine avoiding high-impact situations. But games tend to be a bit more forceful and violent, and that of course means heaps of particles! Particles are a great way to create dynamic and exciting effects that further give impact and weight to actions. 
In Fish Fight, [macroquad-particles]( https://crates.io/crates/macroquad-particles) is used to handle particles. 
