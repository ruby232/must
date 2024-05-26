use log::debug;
use crate::commands::Command;

pub(crate) fn send(command: Command) -> std::io::Result<()> {
	use interprocess::local_socket::{prelude::*, GenericFilePath, GenericNamespaced, Stream};
	use std::io::{prelude::*, BufReader};

	// @todo, Crear el socker en un solo metodo para no repetir codigo
	let name = if GenericNamespaced::is_supported() {
		"must.sock".to_ns_name::<GenericNamespaced>()?
	} else {
		"/tmp/must.sock".to_fs_name::<GenericFilePath>()?
	};

	// @todo, Analizas si es factible crear el buffer en un solo metodo para el servidor y el cliente
	// y/o agregar un parametro para la capacidad como una variable global
	let mut buffer = String::with_capacity(128);

	let conn = Stream::connect(name)?;
	let mut conn = BufReader::new(conn);
	let mut msg = command.to_string();
	msg.push('\n');
	debug!("Sending command: {msg}");
	conn.get_mut().write_all(msg.as_ref())?;
	conn.read_line(&mut buffer)?;
	println!("{}", buffer);
	debug!("Server answered: {buffer}");
	Ok(())
}