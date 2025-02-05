use std::{env, fs::{File, OpenOptions}, io::{BufWriter, ErrorKind, Read, Write}, str::FromStr};
use json::JsonValue;
use chrono::{DateTime, Local};

fn main() {
    let cli = Cli::build(env::args());
    cli.run();
}

enum EnumSubCommands{
    Add(AddCommand),
    Update(UpdateCommand),
    Delete(DeleteCommand),
    MarkInProgress(MarkInProgressCommand),
    MarkDone(MarkInDoneCommand),
    List(ListCommand),
}

struct Cli{
    subcommand: EnumSubCommands,
}
impl Cli{
    fn build(mut args: impl Iterator<Item = String>) -> Self {
        args.next();
        let subcommand = match args.next().unwrap_or_default().as_ref() {
            "add" => EnumSubCommands::Add( AddCommand::build(args) ),
            "update" => EnumSubCommands::Update( UpdateCommand::build(args) ),
            "delete" => EnumSubCommands::Delete( DeleteCommand::build(args) ),
            "mark-in-progress" => EnumSubCommands::MarkInProgress( MarkInProgressCommand::build(args) ),
            "mark-done" => EnumSubCommands::MarkDone( MarkInDoneCommand::build(args) ),
            "list" => EnumSubCommands::List( ListCommand::build(args) ),
            _ => panic!("Hola---"),
        };

        Cli{ subcommand }
    }

    fn run(&self) {
        match &self.subcommand {
            EnumSubCommands::Add(f) => f.run(),
            EnumSubCommands::Update(f) => f.run(),
            EnumSubCommands::Delete(f) => f.run(),
            EnumSubCommands::MarkInProgress(f) => f.run(),
            EnumSubCommands::MarkDone(f) => f.run(),
            EnumSubCommands::List(f) => f.run(),
        };
    }
}

#[derive(Debug, PartialEq)]
enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug)]
struct Task {
    id: u32,
    description: String,
    status: TaskStatus,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>
}

impl Task {
    fn from_json(value: &JsonValue) -> Option<Self> {
        Some(Task {
            id: value["id"].as_str()?.parse().unwrap(),
            description: value["description"].as_str()?.to_string().clone(),
            status: match value["status"].as_str()? {
                "done" => TaskStatus::Done,
                "in-progress" => TaskStatus::InProgress,
                "todo" => TaskStatus::Todo,
                _ => panic!("Error: Corrupted data")
            },
            created_at: match DateTime::from_str(value["createdAt"].as_str()?) {
                Ok(d) => d,
                Err(_) => panic!("No sé")
            },
            updated_at: match DateTime::from_str(value["updatedAt"].as_str()?) {
                Ok(d) => d,
                Err(_) => panic!("No sé")
            },
        })
    }

    fn to_json(&self) -> JsonValue {
        let mut obj = JsonValue::new_object();
        obj["id"] = self.id.to_string().into();
        obj["description"] = self.description.clone().into();
        obj["status"] = match self.status {
            TaskStatus::Done => String::from("done").into(),
            TaskStatus::InProgress => String::from("in-progress").into(),
            TaskStatus::Todo => String::from("todo").into(),
        };
        obj["createdAt"] = self.created_at.to_string().into();
        obj["updatedAt"] = self.updated_at.to_string().into();
        obj
    }
}

trait SubCommand {
    fn build(args: impl Iterator<Item = String>) -> Self;
    fn help() -> String;
    fn run(&self);
}

struct AddCommand {
    description: String,
}

impl SubCommand for AddCommand {
    fn build(mut args: impl Iterator<Item = String>) -> Self{
        let description = args.next().expect(AddCommand::help().as_ref());
        AddCommand{ description }
    }
    fn help() -> String {
        String::from("
        # Something
        ")
    }
    fn run(&self) {
        let data = get_json_from_file();
        let mut new_data: Vec<Task> = data
            .members()
            .filter_map(Task::from_json).collect();
        
        let new_id = if new_data.is_empty() {
            1
        } else {
            new_data.last().unwrap().id + 1
        };
        
        new_data.push(Task {
            id: new_id,
            description: self.description.clone(),
            status: TaskStatus::Todo,
            created_at: Local::now(),
            updated_at: Local::now(),
        });

        let json_data = JsonValue::Array(new_data.iter().map(|value| value.to_json()).collect());
        set_json_2_file(json_data);
    }
}
struct UpdateCommand {
    id: u32,
    description: String,
}

impl SubCommand for UpdateCommand {
    fn build(mut args: impl Iterator<Item = String>) -> Self{
        let id = args.next().expect(AddCommand::help().as_ref()).parse().unwrap();
        let description = args.next().expect(AddCommand::help().as_ref());
        UpdateCommand{ id, description }
    }
    fn help() -> String {
        String::from("
        # Something
        ")
    }
    fn run(&self) {
        let data = get_json_from_file();
        let new_data = data
            .members()
            .filter_map(Task::from_json)
            .map(|mut value| {
                if value.id == self.id {
                    value.description = self.description.clone();
                }
                value
            });
        let json_data = JsonValue::Array(new_data.map(|value| value.to_json()).collect());
        set_json_2_file(json_data);
    }
}
struct DeleteCommand {
    id: u32,
}

impl SubCommand for DeleteCommand {
    fn build(mut args: impl Iterator<Item = String>) -> Self{
        let id = args.next().expect("Forgotten id arg").parse().unwrap();
        DeleteCommand{ id }
    }
    fn help() -> String {
        String::from("
        # Something
        ")
    }
    fn run(&self) {
        let data = get_json_from_file();
        let filtered_data  = data
            .members()
            .filter_map(Task::from_json)
            .filter(|value| value.id != self.id);

        let json_data = JsonValue::Array(filtered_data.map(|value|value.to_json()).collect());
        set_json_2_file(json_data);
    }
}

struct MarkInProgressCommand {
    id: u32,
}

impl SubCommand for MarkInProgressCommand {
    fn build(mut args: impl Iterator<Item = String>) -> Self{
        let id = args.next().expect("Forgotten id arg").parse().unwrap();
        MarkInProgressCommand{ id }
    }
    fn help() -> String {
        String::from("
        # Something
        ")
    }
    fn run(&self) {
        let data = get_json_from_file();
        let new_data = data
            .members()
            .filter_map(Task::from_json)
            .map(|mut value| {
                if value.id == self.id {
                    value.status = TaskStatus::InProgress;
                }
                value
            });
        let json_data = JsonValue::Array(new_data.map(|value| value.to_json()).collect());
        set_json_2_file(json_data);
    }
}

struct MarkInDoneCommand {
    id: u32,
}

impl SubCommand for MarkInDoneCommand {
    fn build(mut args: impl Iterator<Item = String>) -> Self{
        let id = args.next().expect("Forgotten id arg").parse().unwrap();
        MarkInDoneCommand{ id }
    }
    fn help() -> String {
        String::from("
        # Something
        ")
    }
    fn run(&self) {
        let data = get_json_from_file();
        let new_data = data
            .members()
            .filter_map(Task::from_json)
            .map(|mut value| {
                if value.id == self.id {
                    value.status = TaskStatus::Done;
                }
                value
            });
        let json_data = JsonValue::Array(new_data.map(|value| value.to_json()).collect());
        set_json_2_file(json_data);
    }
}

struct ListCommand {
    status: TaskStatus,
}

impl SubCommand for ListCommand {
    fn build(mut args: impl Iterator<Item = String>) -> Self{
        let status = match args.next().expect("Forgotten id arg").as_ref() {
            "done" => TaskStatus::Done,
            "todo" => TaskStatus::Todo,
            "in-progress" => TaskStatus::InProgress,
            _ => panic!("Expected: done, todo, in-progress")
        };
        ListCommand{ status }
    }
    fn help() -> String {
        String::from("
        # Something
        ")
    }
    fn run(&self) {
        let data = get_json_from_file();
        let filtered_data = data
            .members()
            .filter_map(Task::from_json)
            .filter(|value| value.status == self.status);
        
        for value in filtered_data {
            println!("-\tid: {}\n\tdescription: {}\n\tstatus: {:?}\n\tcreatedAt: {:?}\n\tupdatedAt: {:?}\n", value.id, value.description, value.status, value.created_at, value.updated_at)
        }
    }
}

fn get_json_from_file() -> JsonValue{
    let mut f = File::open("data.json").unwrap_or_else(|error| {
        match error.kind(){
            ErrorKind::NotFound => File::create("data.json").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {error:?}");
            }),
            other_error =>    panic!("Problem opening the file: {other_error:?}")
        }
    });

    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap_or_else(|error| {
        panic!("Problem opening the file: {error:?}")
    });

    if contents.is_empty() {
        return JsonValue::new_array();
    }
    json::parse(&contents).unwrap()

}

fn set_json_2_file(content: JsonValue) {
    let f = OpenOptions::new()
        .write(true)   // Habilita escritura
        .create(true)  // Crea el archivo si no existe
        .truncate(true) // Borra el contenido anterior
        .open("data.json")
        .unwrap_or_else(|error| {
            panic!("Problem opening the file: {error:?}");
        });

    let data = json::stringify(content);

    let mut writer = BufWriter::new(f);
    writer.write(data.as_bytes()).unwrap_or_else(|error| {
        match error.kind() {
            ErrorKind::Interrupted => panic!("Process was interrupted: {error:?}"),
            other_error => panic!("Problem writting the file: {other_error:?}")
        };
    });
}