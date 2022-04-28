use crate::model::Database;
use juniper::{FieldResult, graphql_object };
use mongodb::{bson::doc };
use serde::{Serialize,Deserialize};
use futures::stream::{StreamExt, TryStreamExt};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    title: String,
    description: String,
    completed: bool
}

#[juniper::graphql_object(description = "A todo")]
impl Todo{
    pub fn title(&self)->&str{
        self.title.as_str()
    }

    pub fn description(&self)->&str{
        self.description.as_str()
    }

    pub fn completed(&self)->bool{
        self.completed
    }
}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Database)]
impl QueryRoot {
    async fn todos(database: &Database) -> FieldResult<Vec<Todo>> {
        let client = database.connect().await?;    
        let collection = client.database("todos").collection("todos");
        let mut cursor = collection.find(None, None).await?;
        let mut todos = Vec::new();
        
        while let Some(doc) = cursor.try_next().await? {
            todos.push(doc);
        }
        
        return Ok({
            todos
        })
    }
}

pub struct MutationRoot;

#[derive(juniper::GraphQLInputObject,Debug, Clone)]
pub struct NewTodo{
    pub title: String,
    pub description: String,
    pub completed: bool
}

#[juniper::graphql_object(Context = Database)]
impl MutationRoot {
    async fn create_todo(database: &Database, new_todo: NewTodo) -> FieldResult<Todo> {
        let client = database.connect().await?;
        let collection = client.database("todos").collection("todos");
        let todo = doc!{
            "title": new_todo.title,
            "description": new_todo.description,
            "completed": new_todo.completed
        };
        let result = collection.insert_one(todo, None).await?;
        let id = result.inserted_id.as_object_id().unwrap();
        let inserted_todo = collection.find_one(Some(doc!{"_id": id}), None).await.unwrap().unwrap();
        return Ok(Todo{
            title: inserted_todo.get("title").unwrap().as_str().unwrap().to_string(),
            description: inserted_todo.get("description").unwrap().as_str().unwrap().to_string(),
            completed: inserted_todo.get("completed").unwrap().as_bool().unwrap()
        });     
    }
}


// pub type Schema = RootNode<'static, QueryRoot, MutationRoot,>;

// pub fn create_schema() -> Schema {
//     return Schema::new(QueryRoot, MutationRoot);
// }