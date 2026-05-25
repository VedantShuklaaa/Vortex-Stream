let binding = null;

function getBinding() {
	if (!binding) {
		binding = require("./index.js");
	}

	return binding;
}

module.exports = {
	get JsVortexStream() {
		return getBinding().JsVortexStream;
	},
};