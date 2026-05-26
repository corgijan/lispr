use crate::types::Lispex;

/// Core Lispex macro — evaluates a single Lispex expression at compile time.
#[macro_export]
macro_rules! lsp {
    ((+ $a:tt $b:tt)) => {{lsp![$a]+lsp![$b] }};
    ((- $a:tt $b:tt)) => { lsp![$a]-lsp![$b] };
    ((div $a:tt $b:tt)) => { lsp![$a] /lsp![$b] };
    ((eq $a:tt $b:tt)) => { lsp![$a].internal()==lsp![$b].internal() };
    ((mul $a:tt $b:tt)) => { lsp![$a] *lsp![$b] };
    ((print $($a:tt)+ )) =>{println!("{}", lsp![$($a)+])};
    ((dbg $($a:tt)+ )) =>{{let v=lsp![$($a)+];println!("[dbg] {}", v);v}};
    ((atoi $($a:tt)+ )) =>{{
        let v=lsp![$($a)+];
        match v{
            $crate::types::Lispex::STRING(s)|$crate::types::Lispex::ATOM(s)=>
                $crate::types::Lispex::NUMBER(s.parse::<i32>().expect("atoi: invalid integer string")),
            _=>panic!("atoi: expected STRING or ATOM")
        }
    }};
    ((itoa $($a:tt)+ )) =>{{
        let v=lsp![$($a)+];
        if let $crate::types::Lispex::NUMBER(n)=v{$crate::types::Lispex::STRING(n.to_string())}
        else{panic!("itoa: expected NUMBER")}
    }};
    ((first $($a:tt)+))=>{{
        let v=lsp![$($a)+];
        v[0].clone().clone().clone()
        }};

    ((rest $($a:tt)+))=>{{
        let mut v=lsp![$($a)+];
        v.remove(0);
        v
        }};

    (#($($a:tt)+))=>{{
        lsp![(#$($a)+)]
        }};

    ((# $($a:tt)+))=>{{
        let mut v=Vec::new();
        $(v.push(lsp![$a]);)+
        $crate::types::Lispex::LIST(v)
        }};

    ((nth $n:tt $($a:tt)+))=>{{
        let v = lsp![$($a)+];
        v[$n].clone()
        }};
    ((if $cond:tt ($($then:tt)+) ($($else:tt)*) ))=>{
        if lsp![$cond] {
            lsp![($($then)+)]
        }else{
             lsp![($($else)*)]
        }
    };
    ((setq $varname:ident $($more:tt)+))=>{
          let $varname = lsp![$($more)+];
        };

    ((str->chars $s:tt)) => {{
        let s = lsp![$s];
        if let $crate::types::Lispex::STRING(s_str) = s {
            let chars: Vec<$crate::types::Lispex> = s_str.chars()
                .map(|c| $crate::types::Lispex::STRING(c.to_string()))
                .collect();
            $crate::types::Lispex::LIST(chars)
        } else {
            panic!("str->chars expects a string argument")
        }
    }};

    ((chars->str $($a:tt)+)) => {{
        let list = lsp![$($a)+];
        if let $crate::types::Lispex::LIST(items) = list {
            let s: String = items.iter().map(|item| match item {
                $crate::types::Lispex::STRING(s) => s.clone(),
                $crate::types::Lispex::ATOM(a) => a.clone(),
                _ => panic!("chars->str expects a list of strings or atoms"),
            }).collect();
            $crate::types::Lispex::STRING(s)
        } else {
            panic!("chars->str expects a list argument")
        }
    }};

    ((split $s:tt $sep:tt)) => {{
        let s = lsp![$s];
        let sep = lsp![$sep];
        if let $crate::types::Lispex::STRING(sep_str) = sep {
            if let $crate::types::Lispex::STRING(s_str) = s {
                let parts: Vec<$crate::types::Lispex> = s_str.split(&sep_str)
                    .map(|part| $crate::types::Lispex::STRING(part.to_string()))
                    .collect();
                $crate::types::Lispex::LIST(parts)
            } else {
                panic!("split expects the first argument to be a string")
            }
        } else {
            panic!("split separator must be a string")
        }
    }};

    // Create an object: (obj (name "Alex") (age 33))
    ((obj $(($field:ident $($val:tt)+))* )) => {{
        let mut fields: Vec<(String, $crate::types::Lispex)> = Vec::new();
        $(
            fields.push((stringify!($field).to_string(), lsp![$($val)+]));
        )*
        $crate::types::Lispex::OBJECT(fields)
    }};

    // Field access sugar: (. object fieldname) -> (get object fieldname)
    ((. $obj:tt $field:ident)) => {
        lsp![(get $obj $field)]
    };

    // Field access: (get object fieldname)
    ((get $obj:tt $field:ident)) => {{
        let o = lsp![$obj];
        o.get_field(stringify!($field))
    }};

    // Field set: (set! object fieldname value)
    ((set! $obj:ident $field:ident $($val:tt)+)) => {{
        $obj.set_field(stringify!($field), lsp![$($val)+]);
    }};

    ((readfile $filename:tt)) => {{
        use std::fs;
        let f = lsp![$filename];
        if let $crate::types::Lispex::STRING(s) = f {
            let contents = fs::read_to_string(&s)
                .expect(&format!("Error reading file: {}", s));
            $crate::types::Lispex::STRING(contents)
        } else {
            panic!("readfile expects a string argument")
        }
    }};


    ((writefile $filename:tt $content:tt)) => {{
        use std::fs;
        let f = lsp![$filename];
        let c = lsp![$content];
        if let ($crate::types::Lispex::STRING(fname), $crate::types::Lispex::STRING(text)) = (f, c) {
            fs::write(&fname, &text)
                .expect(&format!("Error writing file: {}", fname));
            $crate::types::Lispex::STRING(format!("Wrote {} bytes to {}", text.len(), fname))
        } else {
            panic!("writefile expects (writefile <string-path> <string-content>)")
        }
    }};

    ((appendfile $filename:tt $content:tt)) => {{
        use std::fs::OpenOptions;
        use std::io::Write;
        let f = lsp![$filename];
        let c = lsp![$content];
        if let ($crate::types::Lispex::STRING(fname), $crate::types::Lispex::STRING(text)) = (f, c) {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&fname)
                .expect(&format!("Error opening file for append: {}", fname));
            file.write_all(text.as_bytes())
                .expect(&format!("Error appending to file: {}", fname));
            $crate::types::Lispex::STRING(format!("Appended {} bytes to {}", text.len(), fname))
        } else {
            panic!("appendfile expects (appendfile <string-path> <string-content>)")
        }
    }};

    ((str-starts-with $s:tt $prefix:tt)) => {{
        let s = lsp![$s];
        let prefix = lsp![$prefix];
        if let ($crate::types::Lispex::STRING(s_str), $crate::types::Lispex::STRING(p_str)) = (s, prefix) {
            $crate::types::Lispex::NUMBER(if s_str.starts_with(&*p_str) { 1 } else { 0 })
        } else {
            panic!("str-starts-with expects two string arguments")
        }
    }};

    // For loop — inline literal list: (for var in #(a b c) body)
    ((for $var:ident in #($($items:tt)*) $($body:tt)+)) => {{
        let __list = lsp![(# $($items)*)];
        if let $crate::types::Lispex::LIST(vec) = __list {
            for __item in vec {
                let $var = __item;
                lsp![$($body)+];
            }
        } else {
            panic!("for expects a list as its second argument")
        }
    }};

    // For loop — named variable: (for var in mylist body)
    ((for $var:ident in $list:ident $($body:tt)+)) => {{
        let __list = lsp![$list];
        if let $crate::types::Lispex::LIST(vec) = __list {
            for __item in vec {
                let $var = __item;
                lsp![$($body)+];
            }
        } else {
            panic!("for expects a list as its second argument")
        }
    }};

    // For loop without `in`: (for var list body)
    ((for $var:ident $list:ident $($body:tt)+)) => {{
        let __list = lsp![$list];
        if let $crate::types::Lispex::LIST(vec) = __list {
            for __item in vec {
                let $var = __item;
                lsp![$($body)+];
            }
        } else {
            panic!("for expects a list as its second argument")
        }
    }};

    ((defun $fname:ident ($($paramname:ident)*)  $($body:tt)+))=>{
         #[allow(dead_code)]
         fn $fname($($paramname:&$crate::types::Lispex,)*)->$crate::types::Lispex{
            lsp![$($body)+]
         }
      };

    (($fname:ident $($param:tt)*))=>{
        $fname($(&lsp![$param],)*)
        };

    ($name:lifetime) => {{
        let s = stringify!($name);
        $crate::types::Lispex::ATOM(s[1..].to_string())
    }};

    ($varname: ident) => {
       $varname.clone()
        };

    ($a:expr)=>{$crate::types::Lispex::from($a)};
}

/// Evaluates multiple Lispex expressions sequentially.
#[macro_export]
macro_rules! lsp_program {
    ($($a:tt)+)=>{
        $(lsp![$a];)+
    };
}
macro_rules! lsp_main {
    ($($a:tt)+)=>{
        lsp_stdlib!();
        $(lsp![$a];)+
    };
}
