mod r2d2_connection;
use r2d2_connection::R2D2Connection;

static R2D2_HOSTNAME:&str = "jcwork.local";
static R2D2_PORT:u16 = 1296;

fn main() {
    let connection = R2D2Connection::new(R2D2_HOSTNAME,R2D2_PORT);

}
