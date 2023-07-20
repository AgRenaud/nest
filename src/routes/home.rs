use maud::{DOCTYPE, Markup, html};

pub async fn home() -> Markup {
    html!(
        (DOCTYPE)
        head {
            meta charset="utf-8";
            script src="https://unpkg.com/htmx.org@1.9.3" {};
            title { "Nest" }
        }
        body {
            h1 { "Welcome to nest ! 🪺"}
            form hx-post= "/login" {
                label for="uname" { b { "Username" } }
                input type="text" placeholder="Enter Username" name="uname" required;                
            
                label for="psw" { b { "Password" } }
                input type="password" placeholder="Enter Password" name="psw" required;                
            
                button type="submit" { "Login" }
            }
        }
    )
}
