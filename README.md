# RGB Cal

## Tom Jacobson

### Measurements

### Overview

This tool is designed to find out a decent frame rate and maximum RGB component values to produce a white-looking RGB of reasonable brightness. Code skeleton provided by Bart Massey 2024. Code features extended by Tom Jacobson.
### Writeup
This was a fun assignment as it involved working with a couple of different pieces of external hardware. The wiring itself was not too difficult once I got my bearings, and the tips provided in the assignment spec helped answer most all of my wiring questions. The only trouble I ran into was the nob readings being backwards but that was easily fixed by switching two wires that I had accidentally swapped positions of. 

Overall the coding portion of the assignment went very smoothly. After spending some time with the code base it became fairly clear how things worked, and as they already were programmed to work with the blue LED it was not too complicated to add the logic for the red and green. The framerate was somewhat tricky but not too bad either after a little bit of trial and error. This was a good final assignment as the brunt of the most tedious work was already done for us and allowed us to focus more on interacting with the hardware. It continues to amaze me how syntatically heavy such a simple program can be. Obviously there were some extra steps to make the code extensible and for it to be alterable without significant reworks. Even so, its amazing how something so simple can take multiple files and a couple of hundred lines of code.

#### Sources

- https://github.com/pdx-cs-rust-embedded/hw-rgbcal-skeleton

