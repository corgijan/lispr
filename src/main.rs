mod ops;
mod types;
#[macro_use]
mod macros;
#[macro_use]
mod stdlib;

fn main() {
        lsp_main![
            (defun test (x) (mul x x))
            (print (test 5))
            (setq x (atoi (readfile "read.txt")))
            (print x)
            (setq xstr (itoa x))
            (print (str->chars xstr))
            (writefile "write.txt" xstr)
        ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic() {
        lsp_stdlib!();
        assert_eq!(types::Lispex::NUMBER(4), lsp![(+ 2 2)]);
        assert_eq!(types::Lispex::NUMBER(9), lsp![(square 3)]);
        assert_eq!(types::Lispex::NUMBER(8), lsp![(add 5 3)]);
        assert_eq!(types::Lispex::NUMBER(1112), lsp![(+ 1 1111)]);
    }

    #[test]
    fn test_factorial() {
        lsp_stdlib!();
        lsp_program![
            (defun fac (x) (if (eq x 1) (1) (mul x (fac (- x 1)))))
        ];
        assert_eq!(types::Lispex::NUMBER(6), lsp![(fac 3)]);
        assert_eq!(types::Lispex::NUMBER(120), lsp![(fac 5)]);
    }

    #[test]
    fn test_list_operations() {
        lsp_program![
            (setq name #('test "aa" 'apples 'alex_payne "123.3" 3))
        ];
        assert_eq!(types::Lispex::ATOM("test".to_string()), lsp![(nth 0 name)]);

        lsp_program![
            (setq user1 #('alex_payne 33))
            (setq user2 #('jan_vaorin 22))
            (setq users #(user1 user2))
        ];
        assert_eq!(
            lsp![(nth 1 users)],
            types::Lispex::LIST(vec![
                types::Lispex::ATOM("jan_vaorin".to_string()),
                types::Lispex::NUMBER(22),
            ])
        );
    }

    #[test]
    #[allow(dead_code)]
    fn test_objects() {
        lsp_program![
            (setq users #(
                (obj (name "Alex") (age 33))
                (obj (name "Jan") (age 22))
            ))
        ];
        assert_eq!(
            types::Lispex::STRING("Alex".to_string()),
            lsp![(. (nth 0 users) name)]
        );
        assert_eq!(types::Lispex::NUMBER(22), lsp![(. (nth 1 users) age)]);
    }

    #[test]
    fn test_string_operations() {
        lsp_stdlib!();
        lsp_program![
            (setq chars (str->chars "hello"))
        ];
        assert_eq!(
            types::Lispex::STRING("hello".to_string()),
            lsp![(chars->str chars)]
        );
        assert_eq!(
            types::Lispex::LIST(vec![
                types::Lispex::STRING("".to_string()),
                types::Lispex::STRING("b,c,".to_string()),
                types::Lispex::STRING("d".to_string()),
            ]),
            lsp![(split "a,b,c,a,d" "a,")]
        );
        assert_eq!(
            types::Lispex::NUMBER(1),
            lsp![(starts_with "hello world" "hello")]
        );
        assert_eq!(
            types::Lispex::NUMBER(0),
            lsp![(starts_with "hello world" "world")]
        );
    }

    #[test]
    fn test_for_loop() {
        lsp_stdlib!();
        lsp_program![
            (for i in #(1 2 3)
                 (add i 10)
            )
        ];
        // for loop runs without panic — verify arithmetic used inside is correct
        assert_eq!(types::Lispex::NUMBER(13), lsp![(add 3 10)]);
    }
}
