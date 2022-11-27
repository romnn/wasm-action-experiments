use cfg_if::cfg_if;
use std::panic;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        extern {
        //     #[wasm_bindgen(js_namespace = console)]
        //     fn error(msg: String);

            type Error;

            #[wasm_bindgen(constructor)]
            fn new() -> Error;

            #[wasm_bindgen(structural, method, getter)]
            fn stack(error: &Error) -> String;
        }

        fn hook_impl(info: &panic::PanicInfo) {
            let mut msg = info.to_string();

            // Add the error stack to our message.
            // This ensures that even if the `console` implementation doesn't
            // include stacks for the stack is still available
            // for the user. Additionally, Firefox's console tries to clean up
            // stack traces, and ruins Rust symbols in the process
            // (https://bugzilla.mozilla.org/show_bug.cgi?id=1519569) but since
            // it only touches the logged message's associated stack, and not
            // the message's contents, by including the stack in the message
            // contents we make sure it is available to the user.
            msg.push_str("\n\nStack:\n\n");
            // let e = Error::new();
            let e = Error::new();
            let stack = e.stack();
            msg.push_str(&stack);

// Safari's devtools, on the other hand, _do_ mess with logged
            // messages' contents, so we attempt to break their heuristics for
            // doing that by appending some whitespace.
            // https://github.com/rustwasm/console_error_panic_hook/issues/7
            msg.push_str("\n\n");

            // Finally, log the panic with `console.error`!
            // error(msg);
            // web_sys::console::error(&JsValue::from(msg).into());
            web_sys::console::error_1(&JsValue::from(msg));
        }
    } else {
        use std::io::{self, Write};

        fn hook_impl(info: &panic::PanicInfo) {
            let _ = writeln!(io::stderr(), "{}", info);
        }
    }
}
/// A panic hook for use with [`std::panic::set_hook`]
/// that logs panics into [`console.error`].
///
/// [`std::panic::set_hook`]: fn@std::panic::set_hook
/// [`console.error`]: https://developer.mozilla.org/en-US/docs/Web/API/Console/error
pub fn hook(info: &panic::PanicInfo) {
    hook_impl(info);
}

/// Lazily install the panic hook exactly once.
#[inline]
pub fn set_once() {
    use std::sync::Once;
    static INSTALL_PANIC_HOOK: Once = Once::new();
    INSTALL_PANIC_HOOK.call_once(|| {
        panic::set_hook(Box::new(hook));
    });
}
