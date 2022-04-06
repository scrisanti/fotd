
pub fn main() {

    enum IPApprKind {
        V4(u32,u32,u32,u32),
        V6(String),
    }

    let home_addr = IPApprKind::V6(String::from("107.34.133.108"));
    let work = IPApprKind::V4(162,43,122,49);
}
