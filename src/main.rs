use wasmedge_quickjs::*;

fn main() {
    let mut ctx = Context::new();

    let code = r#"
      import('istro.js').then((istrolid)=>{
        return istrolid.istro()
      })
    "#;

    let p = ctx.eval_global_str(code);
}