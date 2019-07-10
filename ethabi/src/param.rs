//! Function param.
use ParamType;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ParamIr {
	/// Param name.
	pub name: String,
	/// Param type.
	#[serde(rename="type")]
	pub kind: ParamType,
	/// Components type for tuple.
	#[serde(default)]
	pub components: Vec<Param>,
}

/// Function param.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(from = "ParamIr")]
pub struct Param {
	/// Param name.
	pub name: String,
	/// Param type.
	pub kind: ParamType,
	/// Components type for tuple.
	pub components: Vec<Param>
}

impl From<ParamIr> for Param {
	fn from(p: ParamIr) -> Self {
		let kind = match p.kind {
			ParamType::Tuple(_) if p.components.len() > 0  => {
				let params: Vec<ParamType> = p.components.iter().map(|c| c.kind.clone()).collect();
				ParamType::Tuple(params)
			},
			_ => p.kind,
		};

		Param {
			name: p.name,
			kind: kind,
			components: p.components
		}
	}
}

#[cfg(test)]
mod tests {
	use serde_json;
	use {Param, ParamType};

	#[test]
	fn param_deserialization() {
		let s = r#"{
			"name": "foo",
			"type": "address"
		}"#;

		let deserialized: Param = serde_json::from_str(s).unwrap();

		assert_eq!(deserialized, Param {
			name: "foo".to_owned(),
			kind: ParamType::Address,
			components: vec![]
		});
	}

	#[test]
	fn param_deserialization_with_components() {
		let s = r#"{
			"name": "foo",
			"type": "tuple",
			"components": [
				{
					"name": "bar",
					"type": "uint256"
				}
			]
		}"#;

		let deserialized: Param = serde_json::from_str(s).unwrap();

		assert_eq!(deserialized, Param {
			name: "foo".to_owned(),
			kind: ParamType::Tuple(vec![ParamType::Uint(256)]),
			components: vec![
				Param {
					name: "bar".to_owned(),
					kind: ParamType::Uint(256),
					components: vec![]
				}
			]
		});
	}
}
