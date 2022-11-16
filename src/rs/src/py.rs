use crate::game::Game;
use pyo3::prelude::*;

// python class wrapper for game
#[pyclass]
struct PyGame {
    game: Game,
}

#[pymethods]
impl PyGame {
    #[new]
    fn new(width: u16, height: u16) -> Self {
        Self {
            game: Game::new(width, height),
        }
    }

    fn size(&self) -> PyResult<(u16, u16)> {
        Ok((self.game.width(), self.game.height()))
    }

    fn state(&self) -> PyResult<Vec<i8>> {
        Ok(self.game.state())
    }

    fn state_model(&self) -> PyResult<Vec<f32>> {
        Ok(self.game.state_model())
    }

    fn reset(&mut self) {
        self.game.reset();
    }

    fn score(&self) -> PyResult<usize> {
        Ok(self.game.score())
    }

    fn input(&mut self, cmd: u8) {
        self.game.input(cmd);
    }

    fn input_turn(&mut self, cmd: u8) {
        self.game.input_turn(cmd);
    }

    fn tick(&mut self) {
        self.game.tick();
    }

    fn done(&self) -> PyResult<bool> {
        Ok(self.game.is_ended())
    }

    // fn head(&self) -> PyResult<(i16, i16)> {
    //     Ok(self.game.head())
    // }

    // fn food(&self) -> PyResult<(i16, i16)> {
    //     Ok(self.game.food())
    // }

    fn tick_count(&self) -> PyResult<usize> {
        Ok(self.game.tick_count())
    }

    // fn snake_dir(&self) -> PyResult<u8> {
    //     Ok(self.game.snake_dir())
    // }

    // fn snake_body(&self) -> PyResult<Vec<(i16, i16)>> {
    //     Ok(self.game.snake_body())
    // }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn snake(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGame>()?;
    // m.add_function(wrap_pyfunction!(reset, m)?)?;
    // m.add_function(wrap_pyfunction!(state, m)?)?;
    // m.add_function(wrap_pyfunction!(state_model, m)?)?;
    // m.add_function(wrap_pyfunction!(tick, m)?)?;
    // m.add_function(wrap_pyfunction!(score, m)?)?;
    // m.add_function(wrap_pyfunction!(input, m)?)?;
    // m.add_function(wrap_pyfunction!(done, m)?)?;
    // m.add_function(wrap_pyfunction!(tick_count, m)?)?;
    // m.add_function(wrap_pyfunction!(food, m)?)?;
    // m.add_function(wrap_pyfunction!(head, m)?)?;
    // m.add_function(wrap_pyfunction!(input_turn, m)?)?;
    // m.add_function(wrap_pyfunction!(snake_dir, m)?)?;
    // m.add_function(wrap_pyfunction!(snake_body, m)?)?;
    Ok(())
}
