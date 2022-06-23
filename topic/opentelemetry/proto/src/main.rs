
pub mod helloworld {
    // tonic::include_proto!("sensetime.viper.lite.lambda.event");
    tonic::include_proto!("a.b");
}

// pub mod google {
//     tonic::include_proto!("google.protobuf");
// }

use helloworld::HelloRequest;

fn main() {
    println!("Hello, world!");
}
