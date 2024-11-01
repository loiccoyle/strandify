export function drawCanvas(
  canvas: HTMLCanvasElement,
  image: HTMLImageElement | null,
  pegs: Array<{x: number, y: number}>,
  selectionBox?: { x: number, y: number, width: number, height: number } | null
) {
  const ctx = canvas.getContext('2d');
  if (!ctx) return;

  ctx.clearRect(0, 0, canvas.width, canvas.height);

  if (image) {
    ctx.drawImage(image, 0, 0, canvas.width, canvas.height);
  }

  if (selectionBox) {
    ctx.fillStyle = 'rgba(255, 0, 0, 0.2)';
    ctx.fillRect(selectionBox.x, selectionBox.y, selectionBox.width, selectionBox.height);
    ctx.strokeStyle = 'rgba(255, 0, 0, 0.8)';
    ctx.strokeRect(selectionBox.x, selectionBox.y, selectionBox.width, selectionBox.height);
  }

  pegs.forEach(peg => {
    ctx.beginPath();
    ctx.arc(peg.x, peg.y, 5, 0, Math.PI * 2);
    ctx.fillStyle = 'red';
    ctx.fill();
  });
}