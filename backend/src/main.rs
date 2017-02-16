//

// #![feature(proc_macro)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate iron;
#[macro_use]
extern crate router;
#[macro_use]
extern crate mime;

#[macro_use]
extern crate lazy_static;

use std::fmt::{self, Display};
use std::io::Read;
use std::str::FromStr;
use std::sync::Mutex;

use iron::{Iron, IronResult, Request, Response};
use iron::status::Status;
use iron::modifier::Modifier;

use serde::{Deserialize, Deserializer, Error, Serialize, Serializer};

#[derive(Clone, Debug)]
struct Date
{
    year: i32,
    month: i32,
    day: i32,
}

#[derive(Clone, Debug)]
struct DateParseError;

impl FromStr for Date
{
    type Err = DateParseError;

    fn from_str(s: &str) -> Result<Date, DateParseError>
    {
        let parts = s.split('-').collect::<Vec<_>>();
        if parts.len() == 3 {
            Ok(Date {
                year: parts[0].parse::<i32>().map_err(|_| DateParseError)?,
                month: parts[1].parse::<i32>().map_err(|_| DateParseError)?,
                day: parts[2].parse::<i32>().map_err(|_| DateParseError)?,
            })
        } else {
            Err(DateParseError)
        }
    }
}

impl Deserialize for Date
{
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let date_str: String =
            try!(serde::Deserialize::deserialize(deserializer));
        date_str.parse::<Date>()
                .map_err(|_| D::Error::custom("invalid date format"))
    }
}

impl Serialize for Date
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        format!("{}", self).serialize(serializer)
    }
}

#[allow(unused_must_use)]
impl Display for Date
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day);
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Event
{
    id: Option<i32>,
    name: String,
    description: Option<String>,
    date: Option<Date>,
}

impl Event
{
    fn new(id: i32, name: &str, description: &str, date_str: &str) -> Event
    {
        Event {
            id: Some(id),
            name: name.to_owned(),
            description: Some(description.to_owned()),
            date: Some(date_str.parse::<Date>().unwrap()),
        }
    }
}

macro_rules! events {
    [$(($id: expr, $name: expr, $desc: expr, $date_str: expr)),+] => (
        vec![$(Event::new($id, $name, $desc, $date_str)),+]
    )
}

// GET /events         -- Returns all events in a list
// GET /events/<id>    -- Returns the single event  <id>, if exists
// POST /events        -- Creates a new event
// PUT /events         -- Updates multiple events by the "id" field
// PUT /events/<id>    -- Updates the single event  <id>; the "id" field must
//                        match or be NULL
// GET /events         -- Returns all events
// GET /events         -- Returns all events

fn handle_get_many_events(_: &mut Request) -> IronResult<Response>
{

    let json = {
        let state = STATE.lock().unwrap();
        serde_json::to_string_pretty(&state.events).unwrap()
    };
    Ok(Response::with((mime!(Application / Json), Status::Ok, json)))
}

fn handle_get_single_event(_: &mut Request) -> IronResult<Response>
{
    Ok(Response::with((Status::Ok, format!("Hello, world!"))))
}

fn find_next_id(events: &Vec<Event>) -> i32 {
    events.iter()
        .fold(1,
              |id, event| {
                  if let Some(id_) = event.id {
                      if id <= id_ {
                          id_ + 1
                      } else {
                          id
                      }
                  }
                  else {
                      id
                  }
              })
}

#[derive(Clone, Debug, Serialize)]
struct ErrorResponse
{
    #[serde(rename="errorCode")]
    code: i32,
    message: String,
}

impl ErrorResponse
{
    fn new<S>(code: i32, message: S) -> Self
        where S: Into<String>
    {
        ErrorResponse {
            code: code,
            message: message.into(),
        }
    }
}

impl Modifier<Response> for ErrorResponse
{
    fn modify(self, response: &mut Response)
    {
        mime!(Application / Json).modify(response);
        let json = serde_json::to_string_pretty(&self).unwrap();
        json.modify(response);
    }
}

fn handle_post_new_event(request: &mut Request) -> IronResult<Response>
{
    let mut content = String::new();
    match request.body.read_to_string(&mut content) {
        Ok(_) => {
            match serde_json::from_str::<Event>(&content) {
                Ok(mut event) => {
                    let mut state = STATE.lock().unwrap();
                    event.id = Some(find_next_id(&state.events));
                    let json = serde_json::to_string_pretty(&event).unwrap();
                    state.events.push(event);
                    Ok(Response::with((Status::Ok,
                                       mime!(Application/Json),
                                       json)))
                }
                Err(_) => {
                    println!("ERROR: Failed to parse request\n{}", content);
                    Ok(Response::with((Status::BadRequest,
                                       ErrorResponse::new(100, "Invalid request body"))))
                }
            }
        }
        Err(_) => {
            println!("FATAL: Failed to read request body");
            Ok(Response::with(Status::InternalServerError))
        }
    }
}

struct State {
    events: Vec<Event>
}

impl State {
    fn new() -> Self {
        let events = events![
            (1,
             "TIFF",
             "Toronto Film Festival",
             "2015-09-10"),
            (2,
             "The Martian Premiere",
             "The Martian comes to theatres.",
             "2015-10-02"),
            (3,
             "SXSW",
             "Music, film and interactive festival in Austin, TX.",
             "2016-03-11")
        ];
        State {
            events: events,
        }
    }
}

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::new());
}

fn main()
{
    let router = router!{
        get_many_events:  get  "/events"     => handle_get_many_events,
        post_new_event:   post "/events"     => handle_post_new_event,
        get_single_event: get  "/events/:id" => handle_get_single_event,
    };
    Iron::new(router).http("127.0.0.1:8081").unwrap();
}
