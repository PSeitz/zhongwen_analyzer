fn main() -> pyo3::PyResult<()> {
    let tokens = ckip_ws::segment("我愛傳統中文")?;
    println!("{tokens:?}");
    let tokens = ckip_ws::segment(
        "在某一年的十二月，我心情不好、想要去別的城市走走，所以我就坐上火車，從台北來到台南",
    )?;
    println!("{tokens:?}");
    let tokens = ckip_ws::segment("都會告訴爸爸在校發生了什麼事")?;
    println!("{tokens:?}");
    Ok(())
}
