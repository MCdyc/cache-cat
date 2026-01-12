use fory_derive::ForyObject;

#[derive(ForyObject, Debug, PartialEq)]
pub struct PrintTestReq {
    pub message: String,
}

#[derive(ForyObject, Debug, PartialEq)]
pub struct PrintTestRes {
    pub message: String,
}
