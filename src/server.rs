use log::{debug, error, info, trace};
use crate::task_manager::{Task};
use serde_json::from_str;

pub(crate) fn main() -> std::io::Result<()> {
    use interprocess::local_socket::{prelude::*, GenericNamespaced, ListenerOptions, Stream};
    use std::io::{self, prelude::*, BufReader};

    fn handle_error(conn: io::Result<Stream>) -> Option<Stream> {
        match conn {
            Ok(c) => Some(c),
            Err(e) => {
                error!("Incoming connection failed: {e}");
                None
            }
        }
    }

    //@todo, pasar el nombre del soque a una ocnstante global
    let printname = "must.sock";
    let name = printname.to_ns_name::<GenericNamespaced>()?;

    let opts = ListenerOptions::new().name(name);

    let listener = match opts.create_sync() {
        Err(e) if e.kind() == io::ErrorKind::AddrInUse => {
            error!(
				"Error: could not start server because the socket file is occupied. Please check if
				{printname} is in use by another process and try again."
			);
            return Err(e);
        }
        x => x?,
    };

    info!("Server running at {printname}");
    let mut buffer = String::with_capacity(128);

    for conn in listener.incoming().filter_map(handle_error) {
        let mut conn = BufReader::new(conn);
        debug!("Incoming connection!");
        conn.read_line(&mut buffer)?;
        debug!("Client answered: {buffer}");

        let task: Task = from_str(&buffer).unwrap();
        let mut answer: String = task.run();

        answer.push('\n');
        conn.get_mut().write_all(answer.as_ref())?;
        buffer.clear();
    }
    Ok(())
}