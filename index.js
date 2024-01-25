import init, { CanvasDriven } from "./pkg/gui.js";

init().then(async () => {
  const canvas = document.createElement("canvas");
  document.body.appendChild(canvas);
  canvas.width = canvas.offsetWidth * devicePixelRatio;
  canvas.height = canvas.offsetHeight * devicePixelRatio;
  const driven = CanvasDriven.new(canvas);

  driven.evolve();

  while (true) {
    driven.evolve();
    driven.draw();
    await raf();
  }
});

function raf() {
  return new Promise((resolve) => requestAnimationFrame(resolve));
}
