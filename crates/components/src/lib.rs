#![allow(non_snake_case)]
use hypertext::*;
use serde_json::json;

pub fn Layout(inner: impl Renderable) -> impl Renderable {
    rsx! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <meta name="darkreader-lock">
                <link rel="icon" href="/static/img/favicon.svg">
                <title>Div176</title>
                <script src="https://cdn.tailwindcss.com/3.4.15"></script>
                <script>tailwind.config = {json!({
                    "theme": {
                        "screens": {
                            "mobile": {"max": "639px"},
                            "desktop": {"min": "640px"}
                        },
                        "colors": {
                            "green": {
                                "light": "#63C658",
                                "DEFAULT": "#3F9C35",
                                "dark": "#327A2A"
                            },
                            "yellow": "#CC9200",
                            "red": "#D52B1E",
                            "white": {
                                "DEFAULT": "#F1FAF0",
                                "true": "#FFFFFF"
                            },
                            "black": {
                                "DEFAULT": "#252525",
                                "true": "#000000"
                            }
                        }
                    }
                }).to_string()}</script>
            </head>
            <body>
                { inner }
            </body>
        </html>
    }
}
