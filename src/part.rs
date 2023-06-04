pub mod part {

    pub struct Part{
        pub(crate) id:u16,
        pub(crate) description:String,
        pub(crate) num_actual:u8,
        pub(crate) num_expected:u8
    }

    pub fn get_example_parts() ->[Part;2]{
        let new_part_1: Part = Part{ id: 1, description: String::from("Test1"), num_actual: 12, num_expected: 12 };
        let new_part_2: Part = Part{ id: 2, description: String::from("Test2"), num_actual: 8, num_expected: 8 };
        return [new_part_1,new_part_2]
    }
}
