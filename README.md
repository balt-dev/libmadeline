# libmadeline
Programmers don't do things because they're easy. We do them because we _thought_ they'd be easy.
---
Puts the movement code (graphics not included) of Madeline from Celeste into a DLL you can include in just about anything!

https://github.com/user-attachments/assets/7b75f321-514c-443b-b651-516c0d2554fd

## Notes

Only a subset of the possible player states are programmed in (`Normal`, `Dash`, `Climb`, and `Swim`), and
the implementation does not do some assistance that the base game does,
meaning that _this will not run your TAS'es,_ sorry to say - this is more meant for casual play. Among these unimplemented things are holdable objects (e.g. Theo).

Also, climbhopping is unimplemented
because it's janky and annoying and really hard to get right - you instead do a climb jump without spending stamina for it with some extra speed added towards the wall you hopped from.

That being said, all movement tech that doesn't depend on these does work (to my knowledge)!
You can go and do your chained ultras and reverse supers and all that jazz all you want.

Note that I'm not perfect, and there are bugs that I either don't know about, or can't figure out how to fix. PRs are welcome!

## Usage

You can find an example project in the `demo` folder.

In order to get an instance of Madeline, you need to call `CLST_Init`, which will give you a pointer to a Madeline object.

**DO NOT `delete` OR `free()` THIS POINTER.**

When you are done with it, pass it to `CLST_Drop`, which will safely discard it.

In order to update Madeline's state, set the `input` field as needed according to player input,
and then call `CLST_Tick` with the elapsed time since the last tick.

Note that setting `Madeline.position` to non-integer values
may cause undesired behavior - for whatever reason, the game
stores the integer and fractional parts of the position separately.
You can find the fractional part of the position in `rem_position`, but take care to keep this in the range of `-0.5, 0.5`.

## Celeste's licensing


This code is based on Celeste's publicly released Player.cs file, which can be found [here](https://github.com/NoelFB/Celeste/blob/master/Source/Player/Player.cs). Said code is released under the MIT License:

```
MIT License

Copyright (c) 2018 Noel Berry

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## My licensing

This code is licensed under the Mozilla Public License, version 2.0.
Said license can be found in the LICENSE file at the root of this repository.



---

_Made with love by baltdev_
