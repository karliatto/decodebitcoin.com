import { xor } from "./../pkg";
import("./css/index.css");

const buttons = document.querySelectorAll("button");
buttons.forEach((button) => {
  button.addEventListener("click", async (event) => {
    switch (event.target.id) {
      case "xor-bip39-submit-button":
        const firstWord = document.getElementById(
          "bip39-word-input-fist"
        ).value;
        const secondWord = document.getElementById(
          "bip39-word-input-second"
        ).value;
        try {
          const xored = xor(firstWord, secondWord);
          if (xored) {
            document.querySelector(".json-container").classList.add("visible");
            document.getElementById("json-display").textContent = xored;
          }
        } catch (error) {
          console.error(error);
        }

        break;
    }
  });
});
