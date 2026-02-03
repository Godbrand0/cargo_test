


enum status{
    Active,
    Inactive,
    Suspended,
    Graduated,
    Probation,
}
struct Student{
    id: u8,
    name: String,
    age: u32,
    status: status,
}

struct StudentList{
    data: Vec<Student>,
    next_id: u8,
}

impl  StudentList{
    fn new()-> Self{
        Self{
            data: Vec::new(),
            next_id:1,
        }
    }

     fn add_student(&mut self, name: String, age: u32, status: status) -> u8{
        let current_id =self.next_id;

        let student = Student{
            id: current_id,
            name,
            age,
            status,
        };
        self.data.push(student);
        self.next_id += 1;
        current_id
    }

    fn get_all(&self) -> &Vec<Student>{
        &self.data


    }

   fn get_single(&self, id:u8) -> Option<&Student>{
        self.data.iter().find(|student| student.id == id)
    }

    fn update_status(&mut self, id: u8, new_status: status) {
        if let Some(student) = self.data.iter_mut().find(|s| s.id == id) {
            student.status = new_status;
        }
    }
}



fn main(){
    println!("we are good")
}