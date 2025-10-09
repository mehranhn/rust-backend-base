use dto_variants::{variants};

#[variants(
    exclude pub UserCreateDto {
        id,
        first_name,
    };
    include pub UserUpdateDto {
        pub first_name: Option<$ty>,
        pub first_name: Option<$ty>,
        pub last_name: Option<$ty>,
    };
)]
#[derive(Debug)]
// #[variant_include(
//     #[(overwrite | append | remove)(attributes)]
//     [vis] Name([vis] id, u32)
//     [vis] Name {
//         id,
//         first_name: Option<$ty>
//     }
// )]
// #[variant_exclude(UserCreateDto(id, first_name))]
// #[variant_include(
//     #[xyz]
//     pub UserUpdateDto()
// )]
// #[variant_include(
//     #[xyz]
//     pub UserUpdateDto {
//         #[xyz]
//         pub id,
//         pub first_name: Option<$ty>,
//     }
// )]
pub struct UserBaseDto {
    pub id: i32,
    pub phone: String,
    pub first_name: String,
    pub last_name: String,
}

// #[derive(Debug)]
// pub struct UserCreateDto {
//     pub phone: String,
//     pub first_name: String,
//     pub last_name: String,
// }
//
// #[derive(Debug)]
// pub struct UserUpdateDto {
//     pub phone: Option<String>,
//     pub first_name: Option<String>,
//     pub last_name: Option<String>,
// }
