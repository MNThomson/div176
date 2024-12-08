#![allow(non_snake_case)]
use hypertext::*;

pub fn Layout(inner: impl Renderable) -> impl Renderable {
    rsx! {
        <!doctype html>
        <html>
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <meta name="darkreader-lock">
                <title>Div176</title>
            </head>
            <body>
                { inner }
            </body>
        </html>
    }
}
