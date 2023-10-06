pub trait IVehiculee{
    fn roule()-> String;
}

pub struct Velo{
    
}

impl  IVehiculee for Velo{
fn roule()-> String {
    let ss = "roule bien";
    return ss.to_string();
}
}