# lispr

lispr is a Rust macro that tries to implement a small subset of LISPs syntax in Rust. It is neither especially beautiful or efficient since it intended as a personal exercise for me. I will also add folders with small working examples and smaller implementations (eg calculator) later. 
Quotes ' have been replaced by # due to Rusts Internal parsing of tokens. 

EDIT: Recursive definitions work now! (But some arithmetic operations like + and div do not :()
EDIT: ARITHMETIC IS NOW SUPPORTED WHOOO!

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
   // => NUMBER(9) NUMBER(39916800) NUMBER(1112) [NUMBER(1), NUMBER(2), NUMBER(3)]

```


