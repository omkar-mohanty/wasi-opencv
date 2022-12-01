use wasi_opencv;

fn main() {

    wasi_opencv::imgcodecs::imread("./car.png");
}
