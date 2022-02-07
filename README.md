# lispr

lispr is a rust macro that tries to implement a small subset of LISPs syntax in rust. It is neither especially beautiful or efficient since it intended as a personal exercise for me. I will also add folders with small working examples and smaller implementations (eg calculator in lisp syntax) later. 
Quotes ' have been replaced by # due to Rusts Internal parsing of Tokens.

EDIT: Recursive definitions work now! (But some arithmetic operations like + and div do not :()

```rust
 lsp_program![
        (defun square (x) (mul x x))
        (print (square 3))
        (defun fac (x) (if (eq x 1) (1) (mul x (fac (- x 1))) ))
        (print (fac 11))
        (print (if (eq 1 0) (1) (+ 1 1111)))
        (setq a #(1 2 3))
        (print a)
    ];
   // => LISPEX::LIST([LISPEX::NUMBER(1), LISPEX::LIST([NUMBER(1), LISPEX::NUMBER(2), LISPEX::NUMBER(3)])])
```


