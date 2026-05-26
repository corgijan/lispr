/// Standard library — functions defined in Lisp style using the `lsp!` builtins.
///
/// Call `lsp_stdlib!()` at the top of any scope that needs these functions
/// before using them.
#[allow(unused_macros)]
#[allow(unused_allocation)]

#[macro_export]
macro_rules! lsp_stdlib {
    () => {
        lsp_program![
            // Arithmetic wrappers — defined in Lisp using primitive builtins
            (defun add (a b) (+ a b))
            (defun sub (a b) (- a b))
            (defun square (x) (mul x x))

            // String operations
            (defun starts_with (s prefix) (str-starts-with s prefix))
        ];
    };
}
