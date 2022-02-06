# lispr

lispr is a rust macro that tries to implement a small subset of LISPs syntax in rust. It is neither especially beautiful or efficiant since it intended as a personal exercise for me. I will also add folders with small working examples and smaller implementations (eg calculator in lisp syntax) later. 
Quotes ' have been replaced by # due to Rusts Internal parsing of Tokens.

```rust
 lsp_program![
        (setq a #(1 2 3))
        (print a)
        (print #(1 a))
    ];
   // => LISPEX::LIST([LISPEX::NUMBER(1), LISPEX::LIST([NUMBER(1), LISPEX::NUMBER(2), LISPEX::NUMBER(3)])])
```


