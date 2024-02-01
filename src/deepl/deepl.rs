use pyo3::prelude::*;

pub fn call_python_add_function() -> PyResult<String> {
    // Initialize Python interpreter
    Python::with_gil(|py| {
        // Load the module from the .py file
        let my_python = PyModule::from_code(
            py,
            include_str!("pixel_gen\\pixel_character_gen_v2.py"),
            "pixel_character_gen_v2.py",
            "pixel_character_gen_v2",
        )?;

        // Get the `generate_image` function from the module
        let generate_image = my_python.getattr("launch")?;

        // Call the function without any arguments
        let filename = generate_image.call0()?.extract::<String>()?;

        Ok(filename)
    })
}