After cloning, update each of the submodules.
`git submodule update --init --recursive`

Set your toolchain to nightly :
rustup override set nightly

To view docs locally in your browser run :
`cargo doc --open`

## Development Roadmap
- [ ] Finish the HTML parser so that it can generate a basic dom tree.
  - [ ] Optional : Integrate the [html5lib test suite](https://github.com/html5lib/html5lib-tests) into the parser
- [ ] Work on both the renderer and the CSS parser in tandem.
  - [ ] MILESTONE #1 :Render HTML in a window with OpenGL or Vulcan or whatever.
- [ ] Finish the CSS Parser. 
  - [ ] CSS Syntax module.
  - [ ] CSS Object Model module.
- [ ] MILESTONE #2 : Render HTML with CSS applied to it.
- [ ] WASM Time :
  - [ ] Create a WASM sandbox that can execute arbitrary wasm code.
  - [ ] Create an implementation of the Web assembly system interface whereby users can use the standard library in their apps (WASI).
- [ ] Refine the HTML parser so that it conforms to HTML5 as much as possible.

## Dependencies : 
- ash : Used for interfacing with the Vulcan API.