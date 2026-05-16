const {
	JsVortexStream
} = require("./index.node");

const stream =
	new JsVortexStream();

stream.trades(
	"binance",
	"BTCUSDT",
	(trade) => {
		const parsed = JSON.parse(trade);
		console.log(parsed);
	}
);


setInterval(() => { }, 1000);