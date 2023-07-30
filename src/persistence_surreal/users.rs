

#[derive(Clone)]
pub struct Users {
    db: Arc<Surreal<Client>>,
}

pub struct UserError;

impl Users {
    pub fn new(db: Arc<Surreal<Client>>) -> Users {
        Users { db }
    }

    pub fn create_user() -> Result<(), UserError> {
        todo!()
    }
}