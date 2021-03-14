#[macro_export]
macro_rules! pytype {
    (module = $module:literal

    class $name:ident:
        ...) => {
        struct $name(pyo3::PyAny);

        pyo3::pyobject_native_type_core!(
            $name,
            // FIXME: this is wrong for types extending dict etc. which have nontrivial layout.
            // Probably have to query layout at runtime.
            pyo3::ffi::PyObject,
            *$name::type_object_raw(pyo3::Python::assume_gil_acquired()),
            Some($module)
        );

        impl $name {
            fn type_object_raw(py: pyo3::Python) -> *mut pyo3::ffi::PyTypeObject {
                use pyo3::once_cell::GILOnceCell;
                use pyo3::AsPyPointer;
                static TYPE_OBJECT: GILOnceCell<pyo3::Py<pyo3::types::PyType>> =
                    GILOnceCell::new();

                TYPE_OBJECT
                    .get_or_init(py, || {
                        let imp = py
                            .import($module)
                            .expect(concat!("Can not import module: ", stringify!($module)));
                        let cls = imp.getattr(stringify!($name)).expect(concat!(
                            "Can not load class: {}.{}",
                            stringify!($module),
                            ".",
                            stringify!($name)
                        ));

                        cls.extract()
                            .expect("Imported attribute should be a type object")
                    })
                    .as_ptr() as *mut _
            }
        }
    }
}

#[cfg(test)]
mod test {
    use pyo3::prelude::*;

    #[test]
    fn test_use_pytype() {
        Python::with_gil(|py| -> PyResult<()> {
            let module = PyModule::from_code(
                py,
                r#"
class Foo:
    ...
"#,
                file!(),
                "mymodule"
            )?;

            pytype! {
                module = "mymodule"

                class Foo:
                    ...
            }

            let foo: &Foo = module.getattr("Foo")?.call0()?.downcast()?;

            assert_eq!(foo.get_type().name()?, "Foo");

            Ok(())
        }).unwrap();
    }

// FIXME: probably need to use pyobject_native_type! instead of pyobject_native_type_core!
// in order to make this compile successfully.
//
//     #[test]
//     fn test_inherit_pytype() {
//         Python::with_gil(|py| -> PyResult<()> {
//             let module = PyModule::from_code(
//                 py,
//                 r#"
// class Foo:
//     ...
// "#,
//                 file!(),
//                 "mymodule"
//             )?;

//             pytype! {
//                 module = "mymodule"

//                 class Foo:
//                     ...
//             }

//             #[pyclass(extends = Foo)]
//             struct SubFoo {};

//             let subfoo: PyObject = (SubFoo {}).into_py(py);

//             assert_eq!(subfoo.as_ref(py).get_type().name()?, "SubFoo");

//             Ok(())
//         }).unwrap();
//     }
}
