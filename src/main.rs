mod archivos_estaticos;
mod posts;

use actix_web::{get, web, App, HttpResponse, HttpServer};
use archivos_estaticos::leer_archivo_estatico;
use posts::{obtener_todos_los_posts, Post};
use tera::{Context, Tera};

// Es importante notar que para acceder al State que se guardo
// con .app_data, se coloca como parametro en la definicion del metodo
// el web::Data<T> donde T es el tipo de dato que pasamos en .app_data
// Lo genial es que si no necesitamos utilizar Tera, simplemente podemos
// quitarlo de la definicion, es decir, no es obligatorio poner en las
// definicionnes de las funciones todo lo que se ecuentre en el state
#[get("/")]
async fn index(tera: web::Data<Tera>) -> HttpResponse {
    // Ahora creamos el Context para pasarle valores a nuestro template HTML
    // es importante notar que le agregamos un "mut" porque vamos a modificar
    // sus valores
    let mut context = Context::new();
    // Ahora agregamos el {{nombre}} que espera nuestro template
    // para  ello el key debe coincidir con lo especificado en el template
    // en este caso el valor que se pasa a "posts" son todos los Posts que seran serializados por
    // Serde
    let posts: Vec<Post> = obtener_todos_los_posts();
    context.insert("posts", &posts);
    // Ahora hacemos el render de nuestro template, pasando los valores
    // del contexto para ser reemplazados al momento del render.
    let respuesta_html = tera.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(respuesta_html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // Creamos una configuracion de Tera, en la cual se especificamos
        // que todos los  templates se encuetran en la carpeta "templates"
        // en la raiz del proyecto y que ademas dicha carpeta puede contener sub carpetas
        let tera: Tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .service(leer_archivo_estatico)
            // Aca agregamos al "State" nuestra configuracio de tera
            // para agregar al State, utilizamos el metodo .app_data
            // al cual le pasamos de parametro un web::Data para poder
            // manejar de forma segura la memoria junto con la configuracion
            // de tera
            .app_data(web::Data::new(tera))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
