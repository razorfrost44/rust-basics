#[repr(C)]
pub enum MyUnionType {
    Int,
    Float,
    U8,
}

impl std::fmt::Display for MyUnionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyUnionType::Int => write!(f, "Int"),
            MyUnionType::Float => write!(f, "Float"),
            MyUnionType::U8 => write!(f, "U8"),
        }
    }
}

#[repr(C)]
pub union MyUnion {
    pub int_value: i32,
    pub float_value: f32,
    pub u8_value: u8,
}

#[repr(C)]
pub struct MyUnionWrapper {
    pub union_type: MyUnionType,
    pub union_value: MyUnion,
}

impl std::fmt::Display for MyUnionWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match self.union_type {
                MyUnionType::Int => write!(f, "Int: {}", self.union_value.int_value),
                MyUnionType::Float => write!(f, "Float: {}", self.union_value.float_value),
                MyUnionType::U8 => write!(f, "U8: {}", self.union_value.u8_value),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_unt_union() {
        let my_union = MyUnion { int_value: 42 };
        unsafe {
            assert_eq!(my_union.int_value, 42);
        }
    }

    #[test]
    fn create_float_union() {
        let my_union = MyUnion { float_value: 3.14 };
        unsafe {
            assert_eq!(my_union.float_value, 3.14);
        }
    }

    #[test]
    fn create_u8_union() {
        let my_union = MyUnion { u8_value: 255 };
        unsafe {
            assert_eq!(my_union.u8_value, 255);
        }
    }

    #[test]
    fn using_match_on_union_wrapper() {
        let wrappers = vec![
            MyUnionWrapper {
                union_type: MyUnionType::Int,
                union_value: MyUnion { int_value: 42 },
            },
            MyUnionWrapper {
                union_type: MyUnionType::Float,
                union_value: MyUnion { float_value: 3.14 },
            },
            MyUnionWrapper {
                union_type: MyUnionType::U8,
                union_value: MyUnion { u8_value: 255 },
            },
        ];
        for wrapper in wrappers {
            unsafe {
                match wrapper.union_type {
                    MyUnionType::Int => assert_eq!(wrapper.union_value.int_value, 42),
                    MyUnionType::Float => assert_eq!(wrapper.union_value.float_value, 3.14),
                    MyUnionType::U8 => assert_eq!(wrapper.union_value.u8_value, 255),
                }
            }
        }
    }
}
