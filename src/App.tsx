import { useEffect } from "react";

function App() {
  useEffect(() => {
    async function runEffect() {
      const m = await import("rust2js");
      m.greet("hi!");
    }

    runEffect();
  }, []);

  return (
    <div className="App">
      <h1> Hello, World! </h1>
    </div>
  );
}

export default App;
