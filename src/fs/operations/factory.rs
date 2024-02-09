pub struct FsOperationsFactory {
    file_name: String
}

impl FsOperationsFactory {
    type Output: FsOperation;

    pub fn get(name: &str) -> Self::Output {
        match name {
            "read" => todo!(),
            "write" => FsOperations::Write(Write {}),
            "create" => todo!(),
            "delete" => todo!()
        }
    }
}

pub trait FsOperation {
    fn name(&self) -> String;
    fn result(&self) -> anyhow::Result;
}

pub enum FsOperationTypes {
    Read(ReadOperation),
    Write(WriteOperation),
    Create(CreateOperation),
    Delete(DeleteOperation)
}

impl FsOperation for FsOperationTypes {
    fn name(&self) -> String {
        use FsOperationTypes::*;
        match self {
            Read(read) => todo!(),
            Write(write) => todo!(),
            Create(create) => todo!(),
            Delete(delete) => todo!()
        }
    }

    fn result(&self) -> anyhow::Result {
        use FsOperationTypes::*;
        match self {
            Read(read) => todo!(),
            Write(write) => todo!(),
            Create(create) => todo!(),
            Delete(delete) => todo!()
        }
    }
}
