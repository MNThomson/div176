#![allow(non_snake_case)]

use hypertext::*;
use icons::{CalendarIcon, HomeIcon, HoursIcon, ProfileIcon};
use serde_json::json;

mod icons;

pub fn Layout(inner: impl Renderable) -> impl Renderable {
    rsx! {
        {Raw("<!DOCTYPE html>")}
        <html>
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <meta name="darkreader-lock">
                <link rel="icon" href="/static/img/favicon.svg">
                <title>Div176</title>
                <script src="https://cdn.tailwindcss.com/3.4.15"></script>
                <script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.6/dist/htmx.min.js"></script>
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
                            },
                            "neutral": "#a3a3a3",
                        }
                    }
                }).to_string()}</script>
            </head>
            <body class="flex flex-col h-screen bg-white">
                {TopNav()}
                <main class="mobile:overflow-scroll text-black py-4 mobile:px-5 desktop:px-8 max-w-6xl w-full mx-auto">
                    { inner }
                </main>
                {BottomNav()}
            </body>
        </html>
    }
}

pub fn TopNav() -> impl Renderable {
    rsx! {
        <header class="mobile:hidden bg-green w-full">
            <div class="desktop:px-1 flex h-full max-w-6xl mx-auto h-24">
                <img class="h-20 p-1 ml-3" src="/static/img/logo.svg" />
                <div class="pt-3 text-white">
                    <h1 class="desktop:text-4xl mobile:text-2xl font-medium">St. John Ambulance</h1>
                    <p class="text-[12px]">BC & Yukon Council, BGen David Coell Division 176, Victoria</p>
                </div>
            </div>
        </header>
        <nav class="z-50 bg-white shadow-[0px_5px_10px_2px_rgba(0,0,0,0.3)] sticky top-0">
            <div class="mobile:hidden text-white bg-[linear-gradient(180deg,rgb(44,44,44)0%,rgb(44,44,44)50%,rgb(0,0,0)50%,rgb(0,0,0)100%)]">
                <div class="flex justify-between mx-auto max-w-6xl">
                    <div class="flex mx-auto w-full">
                        <a href="/" class="flex items-center justify-center">
                            <img id="navlogo" class="h-5 -translate-y-px pl-2 pr-1" src="/static/img/logo.svg" />
                            <p class="my-auto pr-3 font-semibold">Div176</p>
                        </a>
                        <div class="flex text-white space-x-0.5 *:px-3 *:py-1 *:my-auto hover:*:bg-green">
                            <a href="#events" class="bg-green">Events</a>
                            <a href="#hours" class="">Hours</a>
                            <a href="#volunteers" class="">Volunteers</a>
                        </div>
                    </div>
                    <div class="flex text-white space-x-0.5 *:px-3 *:py-1 *:my-auto hover:*:bg-green">
                        <a href="/user" class="">Account</a>
                    </div>
                </div>
            </div>
        </nav>
        <script>{Raw("
            document.addEventListener('scroll', function() {
                const nav = document.querySelector('nav');
                const navLogo = document.getElementById('navlogo');

                if (nav) {
                    const navPosition = nav.getBoundingClientRect();

                    navLogo.style.visibility = navPosition.top <= 0 ? 'visible' : 'hidden';
                }
            });

            // Initialize on page load
            document.addEventListener('DOMContentLoaded', function() {
                const navLogo = document.getElementById('navlogo');
                if (navLogo) {
                    navLogo.style.visibility = 'hidden';
                }
            });
        ")}</script>
    }
}

pub fn BottomNav() -> impl Renderable {
    rsx! {
            <nav class="desktop:hidden z-50 h-16 px-1 bg-white shadow-[0px_5px_10px_2px_rgba(0,0,0,0.4)] rounded-t-xl grid grid-cols-4 text-sm space-x-0.5 text-neutral *:text-center *:content-center hover:*:text-green hover:*:fill-green *:transition-all *:duration-200 *:flex *:flex-col *:justify-center *:items-center *:space-y-0.5 fill-neutral">
                <a href="#" class="fill-green text-green font-medium">
                    {HomeIcon()}
                    <p>Home</p>
                </a>
                <a href="#" class="">
                    {CalendarIcon()}
                    <p>Events</p>
                </a>
                <a href="#" class="">
                    {HoursIcon()}
                    <p>Hours</p>
                </a>
                <a href="#" class="">
                    {ProfileIcon()}
                    <p>Account</p>
                </a>
            </nav>
    }
}
