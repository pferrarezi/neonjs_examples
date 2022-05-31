use neon::prelude::*;
mod helpers;
use helpers::fib;


fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    let str_js: String = cx.argument::<JsString>(0)?.value(&mut cx);
    let mut owned_string: String = "hello ".to_owned();
    owned_string.push_str(str_js.as_str());
    Ok(cx.string(owned_string))
}

fn soma_foo(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let x: f64 = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let y: f64 = cx.argument::<JsNumber>(1)?.value(&mut cx);

    let res: f64 = x + y;

    Ok(cx.number(res))
}

fn fibonaci(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let n: f64 = cx.argument::<JsNumber>(0)?.value(&mut cx);    
    let f = fib(n as i64) as f64;
    Ok(cx.number(f))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("somaFoo", soma_foo)?;
    cx.export_function("fibonaci", fibonaci)?;
    Ok(())
}
