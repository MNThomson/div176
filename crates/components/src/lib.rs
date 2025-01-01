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
                <link rel="icon" href="/static/img/favicon.svg">
                <title>Div176</title>
                <script src="https://cdn.tailwindcss.com/3.4.15"></script>
                <script>r#"tailwind.config = {}"#</script>
            </head>
            <body>
                { inner }
            </body>
        </html>
    }
}
