import { decode } from "./../pkg";
import("./index.css");

const buttons = document.querySelectorAll("button");
buttons.forEach((button) => {
  button.addEventListener("click", async (event) => {
    switch (event.target.id) {
      case "raw-tx-submit-button":
        const rawTx = document.getElementById("raw-tx-value").value;
        try {
          const decodedTx = decode(rawTx);
          if (decodedTx) {
            document.querySelector(".json-container").classList.add("visible");
            document.getElementById("json-display").textContent = decodedTx;
          }
        } catch (error) {
          console.error(error);
        }

        break;
    }
  });
});
