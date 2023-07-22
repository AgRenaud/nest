use maud::{html, Markup, DOCTYPE};

pub async fn home() -> Markup {
    html!(
        (DOCTYPE)
        head {
            meta charset="utf-8";
            script src="https://unpkg.com/htmx.org@1.9.3" {};
            script src="https://cdn.tailwindcss.com" {};
            title { "Nest" }
        }
        body {
            div class="w-full" {
                h1 class="max-w-xs mb-4 text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl" { "Welcome to nest ! 🪺"}
                form class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4" hx-post= "/login" {
                    div class = "mb-4" {
                        label for="uname" class="block text-gray-700 text-sm font-bold mb-2" { "Username" }
                        input type="text" placeholder="Enter Username" name="uname" required class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline";
                    }

                    div class="mb-6" {
                        label for="psw" class="block text-gray-700 text-sm font-bold mb-2" { "Password" }
                        input type="password" placeholder="Enter Password" name="psw" required class="shadow appearance-none border border-red-500 rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline";
                    }

                    div class="flex items-center justify-between" {
                        button type="submit" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" { "Login" }
                        a class="inline-block align-baseline font-bold text-sm text-blue-500 hover:text-blue-800" href="#" { "Forgot Password?" }
                    }
                }
            }
        }
    )
}
