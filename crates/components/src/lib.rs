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
            <body class="flex flex-col h-screen bg-white">
                {TopNav()}
                <div class="flex flex-1 overflow-hidden">
                    <main class="text-black mobile:pt-4 mobile:px-5 desktop:pt-8 desktop:px-20 max-w-5xl w-full mx-auto">
                        { inner }
                    </main>
                </div>
            </body>
        </html>
    }
}

pub fn TopNav() -> impl Renderable {
    rsx! {
        <nav class="z-50 bg-white shadow-[0px_5px_10px_2px_rgba(0,0,0,0.3)]">
            <div class="text-white bg-black flex">
                <a href="/" class="flex">
                    <img class="h-6 px-1" src="/static/img/logo.svg" />
                    <p class="my-auto pr-3">Div176</p>
                </a>
                <div class="flex text-sm font-medium text-white space-x-0.5 *:px-3 *:py-1 *:my-auto hover:*:bg-green">
                    <a href="#events" class="bg-green">Events</a>
                    <a href="#hours " class="">Hours</a>
                </div>
            </div>
        </nav>
    }
}
