use std::{path::{Component, PathBuf}, str::FromStr};



use normpath::PathExt;
#[test]
fn path_iter() {
    //  -- --nocapture or $env:RUST_TEST_NOCAPTURE=1  close: del env:RUST_TEST_NOCAPTURE
    //C:\Users\loyal\AppData\Local\Google\Chrome\User Data\Default\Network\Cookies
    let path = std::path::PathBuf::from(
        r"C:\Users\loyal\AppData\Local\Google\Chrome\User Data\Default\Network\新建文件夹\666.txt",
    );
    // let mut path = std::path::PathBuf::from(
    //     r"c:\swapfile.sys",
    // );
    println!("{:?}", path);
    let apath = path.normalize().unwrap();
    // let apath = path;
    let apath_str = apath.as_path().to_str().unwrap();
    println!("{}", apath_str);
    // path.components 左往右迭代
    let components = apath.components();
    // let mut iter = cmpts.into_iter();
    for component in components {
        println!("{}", component.as_os_str().to_str().unwrap());
        match component {
            Component::Normal(s) => println!("Normal component: {}", s.to_str().unwrap()),
            Component::Prefix(s) => {
                println!("Prefix component: {}", s.as_os_str().to_str().unwrap())
            }
            Component::RootDir => println!("RootDir component: {:?}", component),
            Component::CurDir => println!("CurDir component: {:?}", component),
            Component::ParentDir => println!("ParentDir component: {:?}", component),
        }
    }
    let components: Vec<_> = apath.components().map(|comp| comp.as_os_str()).collect();
    println!("{:?}", components);

    let components: Vec<_> = apath
        .components()
        .filter(|comp| Component::RootDir != *comp)
        .collect();
    println!("{:?}", components);

    let components: Vec<_> = apath
        .components()
        .filter_map(|comp| {
            if Component::RootDir != comp {
                Some(comp.as_os_str().to_str().unwrap())
            } else {
                None
            }
        })
        .collect();
    println!("{:?}", components);
    println!("{}", components[0]);

    let save_path = "C:\\Users\\Administrator\\Desktop\\";
    
    let output_file_name = "output.txt";
    let p = format!("{save_path}{}{output_file_name}", std::path::MAIN_SEPARATOR_STR);
    println!("{}", p);

    let path: PathBuf = [save_path,&output_file_name].iter().collect();
    println!("{}", path.to_str().unwrap());
    

    let path = [save_path,&output_file_name].iter().collect::<PathBuf>();
    println!("{}", path.to_str().unwrap());

    let path = PathBuf::from_str("C:\\Users\\").unwrap();
    let apath = path.canonicalize().unwrap();
    println!("{}",apath.as_os_str().to_str().unwrap());
    println!("{:?}", apath.components().filter(|comp| match comp {Component::Prefix(_s)=>true,
        _=>false}).collect::<Vec<Component>>());

    let npath = path.normalize().unwrap();
    println!("{}",npath.as_os_str().to_str().unwrap());
    println!("{:?}", npath.components().filter(|comp| match comp {Component::Prefix(_s)=>true,
   _=>false}).collect::<Vec<Component>>());
    // let mut f = File::open("log.txt").unwrap();
    // let mut reader = BufReader::new(f);
    // Ntfs::new(&mut reader);
}
