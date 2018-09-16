use person::Person;
use rocket_contrib::Json;
use rocket::http::RawStr;

#[get("/")]
fn index() -> String {
    format!("{}", Person {
        name: "AAAA",
        age: "1"
    })
}

#[derive(FromForm, Serialize, Deserialize)]
struct Task {
    name: String,
    age: Option<String>,
}

#[get("/hello?<task>")]
fn hello(task: Task) -> Json<Task> {
    Json(task)
}

#[derive(Serialize, Deserialize)]
struct Message {
    contents: String,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Person")]
struct PersonDef {
    name: &'static str,
    age: &'static str
}

#[derive(Serialize)]
struct SerializablePerson {
    #[serde(with = "PersonDef")]
    person: Person,
    id: String
}

#[get("/zzz/<id>")]
fn get_json(id: String) -> Json<SerializablePerson> {

    let person1 = Person {
        name: "AAAA",
        age: "1"
    };

    Json(SerializablePerson {
        person: person1,
        id
    })
}