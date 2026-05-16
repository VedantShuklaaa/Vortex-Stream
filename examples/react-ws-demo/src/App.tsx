import { useEffect, useState } from "react";

interface Trade {
  exchange: string;
  symbol: string;
  event_type: string;
  event_time: string;
  trade_id: string;
  last_price: string;
  quantity: string;
  timestamp: number;
}

function App() {

  const [trades, setTrades] =
    useState<Trade[]>([]);

  useEffect(() => {

    const ws =
      new WebSocket(
        "ws://localhost:8080/ws"
      );

    ws.onmessage =
      (event) => {

        const trade =
          JSON.parse(
            event.data
          );

        setTrades((prev) => [

          trade,

          ...prev.slice(0, 24),
        ]);
      };

  }, []);

  return (

    <div>

      <h1>
        Vortex Stream
      </h1>

      {
        trades.map((trade, index) => (

          <div key={index}>

            <h3>
              {trade.symbol}
            </h3>

            <p>
              Price:
              {" "}
              {trade.last_price}
            </p>

            <p>
              Quantity:
              {" "}
              {trade.quantity}
            </p>

          </div>
        ))
      }

    </div>
  );
}

export default App;