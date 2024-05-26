use log::debug;

pub(crate) fn send(command: &mut String) -> std::io::Result<()> {
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
	command.push('\n');
	conn.get_mut().write_all(command.as_ref())?;
	conn.read_line(&mut buffer)?;
	// @todo, Aqui se trataria la respuesta del comando para mostrarlo pro consola al usuario
	debug!("Server answered: {buffer}");
	Ok(())
}