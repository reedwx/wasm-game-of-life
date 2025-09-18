//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate wasm_game_of_life;
use wasm_bindgen_test::*;
use wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
    universe
}

#[wasm_bindgen_test]
pub fn test_one_tick() {
    let mut input_universe = input_spaceship();
    //expected universe after one tick for spaceship input
    let expected_universe = expected_spaceship();

    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}

//Beacon pattern with period 2
pub fn periodic_universe() -> Universe {
    let mut universe = Universe::new();
    universe.set_height(6);
    universe.set_width(6);
    universe.set_cells(&[
        (1, 1),
        (1, 2),
        (2, 1),
        (2, 2),
        (3, 3),
        (3, 4),
        (4, 3),
        (4, 4),
    ]);
    universe
}

#[wasm_bindgen_test]
pub fn test_periodic_tick() {
    let mut input_universe = periodic_universe();
    let checked_universe = periodic_universe();

    input_universe.tick();
    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &checked_universe.get_cells());
    input_universe.tick();
    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &checked_universe.get_cells());
}

pub fn static_universe() -> Universe {
    let mut static_universe = Universe::new();
    static_universe.set_height(5);
    static_universe.set_width(6);
    static_universe.set_cells(&[(1, 2), (1, 3), (2, 1), (2, 4), (3, 2), (3, 3)]);
    static_universe
}

#[wasm_bindgen_test]
pub fn test_static_universe() {
    let mut static_u = static_universe();
    static_u.tick();
    let expected_universe = static_universe();
    assert_eq!(&static_u.get_cells(), &expected_universe.get_cells());
}

pub fn large_periodic_universe() -> Universe {
    let mut penta_deca = Universe::new();
    penta_deca.set_height(18);
    penta_deca.set_width(11);
    penta_deca.set_cells(&[
        (4, 4),
        (4, 5),
        (4, 6),
        (5, 3),
        (5, 7),
        (6, 2),
        (6, 8),
        (8, 1),
        (8, 9),
        (9, 1),
        (9, 9),
        (11, 2),
        (11, 8),
        (12, 3),
        (12, 7),
        (13, 4),
        (13, 5),
        (13, 6),
    ]);
    penta_deca
}
/// Tests that the periodicity of the penta-decathlon Game of Life pattern
/// is upheld + live cell count is 15
///
#[wasm_bindgen_test]
pub fn test_large_periodic_universe() {
    let mut penta_deca = large_periodic_universe();
    penta_deca.tick();
    penta_deca.tick();
    penta_deca.tick();
    penta_deca.tick();
    penta_deca.tick();
    penta_deca.tick();
    penta_deca.tick();
    penta_deca.tick();
    penta_deca.tick();
    penta_deca.tick();
    penta_deca.tick();
    penta_deca.tick();
    penta_deca.tick();
    penta_deca.tick();
    penta_deca.tick();
    assert_eq!(
        &penta_deca.get_cells(),
        &large_periodic_universe().get_cells()
    );
}
