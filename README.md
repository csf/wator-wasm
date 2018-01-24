# Implementation of [Wa-Tor](https://en.wikipedia.org/wiki/Wa-Tor) in Rust and WebAssembly. 

Wa-Tor simulates ecological cycles between fish and sharks on a water-covered toroidal planet. By setting up the number of fish and sharks, the breeding times of each, and the starvation time for sharks, the simulation progresses stepwise with fish randomly moving and reproducing, and sharks "hunting" or randomly moving around a wrapping grid representing Wa-Tor's toroidal surface.

I first saw Wa-Tor described in A.K. Dewdney's [Computer Recreations article](http://home.cc.gatech.edu/biocs1/uploads/2/wator_dewdney.pdf) in the December 1984 issue of _Scientific American_ and it was the first non-trivial program I ever wrote from specs on the Apple IIc computer I finally was able to buy a few months later with money I'd saved.

Both the disk with my original source (and the Apple IIc itself) are long gone, so I set out to recreate it for multiple reasons--one part nostalgia, one part in support of an ongoing writing project, and one part in support of a January 2018 tech talk. If Rust and WbAssembly aren't your thing, you can find a  recreation of the Applesoft Basic version [in the wator-basic repo](https://github.com/csf/wator-basic).

## Screenshot
![alt text](https://github.com/csf/wator-wasm/raw/master/images/wator-screen.png "Wa-Tor WebAssembly Screenshot")

## Run It
You can either [run it here in your browser](https://csf.github.io/wator-wasm/index.html) or clone, build, and run it yourself following instructions at [Hello, Rust!](https://hellorust.com/), navigate to the html directory, and serve it up with something like `python -m http.server 9000` or however you want to deploy.

More specifically, to build once you have rust and wasm-gc (optional, but shrinks the `.wasm` file size) installed, navigate to the root and do this:

    cargo build --release --target=wasm32-unknown-unknown
    wasm-gc target/wasm32-unknown-unknown/release/wator.wasm html/wator.wasm

For best results when picking the parameters to run with, you typically want to use something where fish breed faster than sharks, and sharks have to eat before they can spawn.

Unlike the BASIC version, this one just has the sharks and fish look up/down/left/right for move or eat targets, rather than in diagonal cells as well. This conforms to Dewdney's original spec, is a bit faster, and is easily changed.

## What's Wonky and TODOs
It's not the most efficient Rust here; the ParticleList implementation is slower than it needs to be based on my original assumption that sparse arrays might offer some benefit. It still runs at 50+ fps, but uses more CPU than it needs to.

Likewise, there must be a better way than making 5000 calls per frame back into JavaScript for the rendering--still learning.

There's definitely a better way to deal with the random number generator so it doesn't have to be constantly recreated and reseeded, butI believe that'll involve another `Mutex` to keep it out of the `WorldState` (to avoid field-level borrowck issues I ran into) and still make it possible to generate a random initial state with the seed provided by JavaScript. One of those "Tired, But Strong" shortcuts as the talk deadline loomed. Still looking for the canonical example of how best to do this to emerge--I've seen three different approaches.

The program currently doesn't terminate if either the fish or sharks "win."

I've got some presentation sizing inside the Rust code that'd be better off in the JavaScript code.

Finally, the FPS counter needs a fix since it can skew when the browser stops giving animation frames, but the math doesn't take these pauses into account. Should be an easy fix.

## Credits and Other Stuff You Should Check out
* Main game loop idea came from Adolfo Ochagavía's [rocket-wasm](https://github.com/aochagavia/rocket_wasm) demo, which made a lot of things click for me in trying to figure out the interaction of Rust and JavaScript.
* Because in the wasm32-unknown-unknown target, Rust isn't allowed to access underlying native platform services such as sources of randomness, it was helpful to find Colin Eberhardt's [post](https://colineberhardt.github.io/wasm-rust-chip8/web/) on his WebAssembly CHIP-8 emulator written in Rust. Check out the emulator code at [wasm-rust-chip8](https://github.com/ColinEberhardt/wasm-rust-chip8). 
* Finally, thanks to [Steve Crooks](https://github.com/scrooks) for patching some of the form and canvas code at the last minute before my talk.
