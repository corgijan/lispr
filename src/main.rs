 use std::fmt::Debug;
// 1.4.0
use std::ops::{Add,Sub,Div,Mul, Index};



macro_rules! lsp {
    ((+ $a:tt $b:tt)) => {{lsp![$a]+lsp![$b] }};
    ((- $a:tt $b:tt)) => { lsp![$a]-lsp![$b] };
    ((div $a:tt $b:tt)) => { lsp![$a] /lsp![$b] };
    ((eq $a:tt $b:tt)) => { lsp![$a]==lsp![$b] };
    ((mul $a:tt $b:tt)) => { lsp![$a] *lsp![$b] };
    ((print $($a:tt)+ )) =>{println!("{:?}", lsp![$($a)+])};
    ((first $($a:tt)+))=>{{
        let v=lsp![$($a)+];
        v[0]
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
        v
        }};

    ((nth $n:tt $($a:tt)+))=>{{
        let v = lsp![$($a)+];
        v[$n].clone()
        }};
    ((ifff $cond:tt ($($then:tt)+) ($($else:tt)*) ))=>{
        if lsp![ $cond] {
            lsp![($($then)+)]
        }else{
             lsp![($($else)*)]
        }
    };
    ((setq $varname:ident $($more:tt)+))=>{
          let $varname = lsp![$($more)+];  
        };

    ((defun $fname:ident ($($paramname:ident)*)  $($body:tt)+))=>{
         let $fname = |$($paramname,)*|{lsp![$($body)+]};
      };

    (($fname:ident $($param:tt)*))=>{
        $fname($(lsp![$param],)*)
      };

    ($varname: ident) => {
        $varname
        };

    ($a:expr)=>{$a};
    
}

macro_rules! lsp_program {
    ($($a:tt)+)=>{
        $(lsp![$a];)+
    };
}



fn main() {
    lsp_program![
       // (defun square (x) (mul x x))
        //(print (square 3))
        (print (ifff (eq 1 0) (1) (+ 1 1111)))
      //  (setq a #(1 2 3))
       // (print a)
    ];
// assert_eq!(3,(lsp![(nth 0 (rest #(2 3 5)))]));   
}






#[test]
fn it_works() {
    assert_eq!(4, lsp![(+ 2 2)]);
    assert_eq!(1,lsp![(first #(1 2 3))]);
    assert_eq!(vec![2,3],lsp![(rest #(1 2 3))]);
    assert_eq!(vec![2,3],lsp![#(2 3)]);
    assert_eq!(3,lsp![(nth 0 (rest #(2 3 5)))])
}
