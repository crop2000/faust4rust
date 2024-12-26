# faust4rust
I directly upstreamed a lot of the things so this here is mostly random stuff

tools for using and developing faust dsp in rust
currently this is my playground.

# todo in faust

- [x] implement compute_array function
- [x] provide constants by default
- [x] treat FaustDsp as Interface
- [x] implement `-ec` flag for having controls in struct
- [x] implement `-cm` flag for outputs as buses
- [x] implement `-os` flag for one-sample compute function
    - i implemented it in a way that also procudces a compute function but this is actually not really needed that should be taken care of by my own generate stuff.
    - when i remember right the problem with kr only dsps is that the function is reduced to nil so that also the attached kr elements removed.
- [x] implement `-rnt` flag to remove FaustDsp implementation
- [x] benchmark for vectors and slices and arrays have shown that those differences are optimized away.
- [x] implemented a generic compute interface
- [ ] upstream generic compute interface for FaustDsp
- [ ] bargraphs are computed inefficently would be nice if they would be treated more like input controls (difficult to upstream)
    - [ ] bargraphs could lower the variability of their unshared inputs (not easy)
    - [ ] bargraphs shortnames are not always unique
    - [ ] feedback could be implemented differently so that only the "last" is stored in the struct
    - [ ] controls could be removed from the struct and be passed via compute
 
# todo

- [ ] faust-test
    - [x] poc code generation
    - [ ] setup code generation
    - [x] for faust-xml
    - [ ] for different faust flags
    - [ ] for `-ec` flag
    - [ ] for `-cm` flag
- [x] faust-xml/faust-json
    - [x] poc
    - [x] as lib (initial version)
    - [x] moved to rust-faust
- [x] add varname to json to make xml obsolete
- [ ] faust-benchmark
- [x] faust-build allow no architecture file.
- [x] faust-new-ui (wip on rust-faust)
    - [x] faust-macro generate safe enum and related abstractions
    - [x] make architecture files obsolete use rust quote macro

# Licence

I use a strict licence here.
If code get upstreamed it will be relicensed to the upstream project.
