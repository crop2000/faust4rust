# faust4rust
tools for using and developing faust dsp in rust   
currently this is my playground.

# todo in faust

- [x] implement compute_array function
- [x] provide constants by default
- [x] treat FaustDsp as Interface
- [ ] new should be default
- [ ] implement `-ec` flag for having controls in struct
- [ ] implement `-cm` flag for outputs as buses
- [ ] implement inlineable a one-sample compute function
    - [ ] without `-ec`
    - [ ] with `-ec` 
- [ ] provide compute functions for various kinds of data 
    - [ ] implement
        - [ ] vectors of vectors
        - [ ] arrays of arrays
        - [ ] fixed length slices
        - [ ] iterators
        - [ ] zipped iterators
    - [ ] benchmark
    - [ ] upstream

# todo

- [ ] faust-test
    - [x] poc code generation
    - [ ] setup code generation
    - [ ] for faust-xml
    - [ ] for different faust flags
    - [ ] for `-ec` flag
    - [ ] for `-cm` flag
- [ ] faust-xml
    - [x] poc
    - [ ] as lib
- [ ] faust-benchmark
- [ ] faust-new-ui
- [ ] faust-macro make use of out dir for generated code
- [ ] faust-build allow no architecture file.

# Licence

I use a strict licence here.
If code get upstreamed it will be relicensed to the upstream project.
Because of that I will not accept PRs here.
