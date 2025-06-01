// SPDX-License-Identifier: MIT

mod macros;
mod repr;

#[allow(unused_imports)]
use std::{
	fmt::{
		Error as FmtError,
		Write as FmtWrite,
	},
	io::{
		stdout,
		Error as IoError,
		Stdout,
		Write as IoWrite,
	}
};
use mlua::prelude::*;

fn repr(lua: &Lua, (value, maybe_options): (LuaValue, Option<LuaTable>)) -> LuaResult<String> {
	let options: LuaTable = match maybe_options {
		Some(tb) => tb,
		None => lua.create_table()?,
	};

	let color: bool = options.get::<Option<bool>>("color")?.unwrap_or(false);
	let multiline: bool = options.get::<Option<bool>>("multiline")?.unwrap_or(false);
	#[allow(unused_variables)]
	let indent_width: u8 = options.get::<Option<u8>>("indent_width")?.unwrap_or(4u8);

	Ok(match value {
		LuaValue::Nil => if color {
			"\x1b[3;31mnil\x1b[23;39m"
		} else {
			"nil"
		}.to_string(),
		LuaValue::Boolean(inner) => if color {
			if inner {
				"\x1b[1;32mtrue\x1b[22;39m"
			} else {
				"\x1b[1;31mfalse\x1b[22;39m"
			}
		} else if inner {
			"true"
		} else {
			"false"
		}.to_string(),
		LuaValue::Integer(inner) => if color {
			format!("\x1b[34m{inner}\x1b[39m")
		} else {
			inner.to_string()
		},
		LuaValue::Number(inner) => if color {
			format!("\x1b[34m{inner}\x1b[39m")
		} else {
			inner.to_string()
		},
		LuaValue::String(inner) => {
			let mut buffer: Vec<u8> = Vec::new();

			if color {
				b"\x1b[33m".iter().for_each(|byte| buffer.push(*byte));
			}

			buffer.push(b'"');

			for byte_ref in &inner.as_bytes() {
				let byte: u8 = *byte_ref;
				if ((byte < 0x20) && !(multiline && (byte == b'\n')) && byte != b'\t') || (byte == 0x7F) {
					// https://en.wikipedia.org/wiki/Control_Pictures
					buffer.push(0xE2);
					buffer.push(0x90);
					buffer.push(0x80 + byte);
				} else {
					if multiline && (byte == b'\n') {
						buffer.push(b'\\');
					}
					buffer.push(byte);
				}
			}

			buffer.push(b'"');

			if color {
				b"\x1b[39m".iter().for_each(|byte| buffer.push(*byte));
			}

			String::from_utf8(buffer).map_err(|_| {
				LuaError::RuntimeError("".to_string())
			})?
		},
		LuaValue::Function(_inner) => {
			todo!("dope.repr() for functions")
		},
		LuaValue::Thread(_inner) => {
			todo!("dope.repr() for threads")
		},
		LuaValue::Table(_inner) => {
			todo!("dope.repr() for tables")
		},
		LuaValue::UserData(_inner) => {
			todo!("dope.repr() for userdata-objects")
		},
		LuaValue::LightUserData(_inner) => {
			todo!("dope.repr() for lightuserdata-objects")
		},
		/* LuaValue::Error(inner) => {
			todo!("dope.repr() for errors")
		},
		LuaValue::Other(inner) => {
			todo!("dope.repr() for other objects")
		}, */
		_ => return Err(LuaError::RuntimeError(
			"Cannot generate repr for value!".to_string()
		)),
	})
}

fn print(lua: &Lua, args: LuaMultiValue) -> LuaResult<()> {
	let mut writer: Stdout = stdout();

	let options: Option<LuaTable> = Some(table!(lua, {
		color = true,
		multiline = true,
	})?);

	let length: usize = args.len();
	for (i, value) in args.iter().enumerate() {
		write!(writer, "{}", repr(lua, (value.to_owned(), options.to_owned()))?)?;

		if (i + 1) < length {
			write!(writer, " ")?;
		}
	}

	assert!(writer.write(b"\n")? > 0);
	writer.flush()?;

	Ok(())
}

#[mlua::lua_module]
fn dope(lua: &Lua) -> LuaResult<LuaTable> {
	table!(lua, {
		repr = lua.create_function(repr)?,
		print = lua.create_function(print)?,
	}, {
		__name = "dope",
	})
}
