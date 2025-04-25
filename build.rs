fn main() {
    println!("cargo:rustc-link-search=native=C:/SDL2/SDL2-2.32.4/lib/x64");
    println!("cargo:rustc-link-lib=SDL2");

     // SDL2_image
     println!("cargo:rustc-link-search=native=C:/SDL2_image/SDL2_image-2.8.8/lib/x64");
     println!("cargo:rustc-link-lib=SDL2_image");
}
