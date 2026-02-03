

struct Todo{
    id: u32,
    description: String,
    is_completed: bool,
}

struct TodoList{
    data: Vec<Todo>,
    next_id: u8,
}

impl TodoList{
    fn new()->Self{
    Self{
        data: Vec::new(),
        next_id:1,
    }
    }

    fn create_todo(&mut self, title:string) -> u8{
        let current_id =self.next_id;


        let todo = Todo{
            id: current_id,
            description: title,
            is_completed: false,
        };
        self.data.push(todo);
        self.next_id += 1;
        current_id
    }

    fn get_all(&self) -> &Vec<Todo>{
        &self.data
    }

    fn get_single(&self, id:u8) -> Option<&Todo>{
        self.data.iter().find(|todo| todo.id == id).expect("failed")
    }

    fn update_status(&mut self, id:u8, is_completed:bool) -> Option<()>{
        self.data.iter_mut().find(|todo| todo.id == id).map(|todo| todo.is_completed = is_completed)
    }                   

    fn update_title(&mut self, id:u8, title:String) -> Option<()>{
        self.data.iter_mut().find(|todo| todo.id == id).map(|todo| todo.description = title)
    }

    fn update2(&mut self, id:u8, title:String, is_completed:bool) -> Option<()>{
        self.data.iter_mut().find(|todo| todo.id == id).map(|todo| {
            todo.description = title;
            todo.is_completed = is_completed;
        })
    }

    fn update2(&mut self, id:u8, title:String) -> bool{
        self.data.iter_mut().find(|todo| todo.id == id).map(|todo| {
            todo.description = title;
            todo.is_completed = is_completed;
        })
    }

}


fn main(){

}