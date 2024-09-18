import "./style.css";
import {
  lineCoords,
  rectangleCoords,
  circleCoords,
  computeSvg,
  Peg,
  Yarn,
  PatherConfig,
  EarlyStopConfig,
} from "strandify-wasm";

const smallScreen = () => window.innerWidth < 1000;
const max_width = () => window.innerWidth * (smallScreen() ? 0.9 : 0.7);
const max_height = () => window.innerHeight * 0.7;

// Constants
const BRUSH_TYPES = {
  SINGLE: "single",
  LINE: "line",
  BOX: "box",
  CIRCLE: "circle",
  ERASER: "eraser",
};

// State
let state = {
  image: null,
  imageData: null,
  pegs: [],
  isDragging: false,
  draggedPegIndex: -1,
  isDrawing: false,
  isErasing: false,
  startX: 0,
  startY: 0,
};

// DOM Elements
const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");
const imageUpload = document.getElementById("imageUpload");
const brushTypeSelect = document.getElementById("brushType");
const pegCountInput = document.getElementById("pegCount");
const pegClearBtn = document.getElementById("pegClearBtn");
const pegAutoBtn = document.getElementById("pegAutoBtn");
const runBtn = document.getElementById("run");

function resizeCanvas() {
  const widthPrev = canvas.width;
  const heightPrev = canvas.height;

  // If an image was chosen, take it's aspect ratio into account
  const aspectRatio = state.image ? state.image.width / state.image.height : 1;
  // Set canvas width to a percentage of the window width
  if (state.image) {
    canvas.width = Math.min(max_width(), state.image.width);
    canvas.height = canvas.width / aspectRatio;
  } else {
    canvas.width = max_width();
    canvas.height = max_height();
  }

  // Also update the peg positions
  state.pegs.forEach((peg) => {
    peg.x = (peg.x / widthPrev) * canvas.width;
    peg.y = (peg.y / heightPrev) * canvas.height;
  });
  drawCanvas();
  setImageData();
}

document.getElementById("removeImage").addEventListener("click", function () {
  state.image = null;
  document.getElementById("imageUpload").value = ""; // Clear the file input
  // canvas.width = 600;
  // canvas.height = 600;
  drawCanvas();
  runBtn.disabled = true;
});

async function run() {
  if (state.pegs.length == 0) {
    alert("Please add some pegs first");
    return;
  }
  const config = {
    iterations: parseInt(document.getElementById("iterations").value),
    patherYarn: {
      width: parseFloat(document.getElementById("patherYarnWidth").value),
      opacity: parseFloat(document.getElementById("patherYarnOpacity").value),
    },
    yarn: {
      width: parseFloat(document.getElementById("yarnWidth").value),
      opacity: parseFloat(document.getElementById("yarnOpacity").value),
      color: hexToRgb(document.getElementById("yarnColor").value),
    },
    early_stop: {
      loss_threshold: document.getElementById("lossThreshold").value
        ? parseFloat(document.getElementById("lossThreshold").value)
        : null,
      max_count: parseInt(document.getElementById("maxCount").value),
    },
    start_peg_radius: 10,
    skip_peg_within: parseInt(document.getElementById("skipPegWithin").value),
    beam_width: parseInt(document.getElementById("beamWidth").value),
  };

  console.log("config:", config);
  let earlyStopConfig = new EarlyStopConfig(
    config.early_stop.loss_threshold,
    config.early_stop.max_count,
  );
  let yarn = new Yarn(
    config.yarn.width,
    config.yarn.opacity,
    config.yarn.color[0],
    config.yarn.color[1],
    config.yarn.color[2],
  );
  let patherConfig = new PatherConfig(
    config.iterations,
    new Yarn(config.patherYarn.width, config.patherYarn.opacity, 0, 0, 0),
    earlyStopConfig,
    config.start_peg_radius,
    config.skip_peg_within,
    config.beam_width,
  );

  let computePegs = state.pegs.map((peg) => new Peg(peg.x, peg.y));
  // Show loading indicator
  const svgContainer = document.getElementById("svg-container");
  const loadingIndicator = document.getElementById("loading-indicator");

  loadingIndicator.style.display = "inline-flex";
  runBtn.disabled = true;

  // run in background
  try {
    // Run computeSvg in the background
    const svg = await new Promise((resolve) => {
      setTimeout(() => {
        resolve(computeSvg(state.imageData, computePegs, patherConfig, yarn));
      }, 0); // Non-blocking computation
    });

    // Once complete, hide the loading indicator
    document.getElementById("loading-indicator").style.display = "none";

    // Add SVG to the page
    svgContainer.innerHTML = svg;

    // Add download button below the SVG
    const downloadBtn = document.createElement("button");
    downloadBtn.id = "downloadBtn";
    downloadBtn.innerHTML = "Download SVG";
    downloadBtn.addEventListener("click", function () {
      downloadSvg(svg);
    });
    svgContainer.appendChild(downloadBtn);
    svgContainer.style.display = "flex";
  } catch (error) {
    console.error("Error computing SVG:", error);
  }
  loadingIndicator.style.display = "none";
  runBtn.disabled = false;
}

function downloadSvg(svg) {
  const blob = new Blob([svg], { type: "image/svg+xml;charset=utf-8" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = "strandify.svg";
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
}

function hexToRgb(hex) {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result
    ? [
        parseInt(result[1], 16),
        parseInt(result[2], 16),
        parseInt(result[3], 16),
      ]
    : null;
}

// Initialize
async function initialize() {
  resizeCanvas();
  setupEventListeners();
}

function setupEventListeners() {
  document.addEventListener("mousemove", onMouseMove);
  imageUpload.addEventListener("change", handleImageUpload);
  canvas.addEventListener("mousedown", handleCanvasMouseDown);
  pegClearBtn.addEventListener("click", clearPegs);
  pegAutoBtn.addEventListener("click", autoPegs);
  document.addEventListener("mouseup", onMouseUp);
  runBtn.addEventListener("click", run);
  window.addEventListener("resize", resizeCanvas);
}

function setImageData() {
  canvas.toBlob((blob) =>
    blob.arrayBuffer().then((array) => {
      state.imageData = new Uint8Array(array);
    }),
  );
}

// Image Handling
function handleImageUpload(e) {
  clearPegs();
  const file = e.target.files[0];
  const reader = new FileReader();

  reader.onload = (event) => {
    state.image = new Image();
    state.image.onload = () => {
      let { width, height } = state.image;

      // Calculate aspect ratio and scale the image
      const aspectRatio = width / height;

      if (width > max_width() || height > max_height()) {
        if (aspectRatio > 1) {
          // Landscape
          width = max_width();
          height = width / aspectRatio;
        } else {
          // Portrait
          height = max_height();
          width = height * aspectRatio;
        }
      }

      canvas.width = width;
      canvas.height = height;

      drawCanvas();
      setImageData();
    };
    state.image.src = event.target.result;
  };

  reader.readAsDataURL(file);
  runBtn.disabled = false;
}

// Drawing Functions
function drawCanvas() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  if (state.image) {
    ctx.drawImage(state.image, 0, 0, canvas.width, canvas.height);
  }
  drawPegs();
}

function drawPegs() {
  state.pegs.forEach((peg) => {
    ctx.beginPath();
    ctx.arc(peg.x, peg.y, 5, 0, Math.PI * 2);
    ctx.fillStyle = "red";
    ctx.fill();
  });
}

// Mouse Event Handlers
function handleCanvasMouseDown(e) {
  const rect = canvas.getBoundingClientRect();
  state.startX = Math.max(e.clientX - rect.left, 0);
  state.startY = Math.max(e.clientY - rect.top, 0);

  if (brushTypeSelect.value !== BRUSH_TYPES.ERASER) {
    const draggedPeg = findDraggedPeg(state.startX, state.startY);
    if (draggedPeg !== -1) {
      state.isDragging = true;
      state.draggedPegIndex = draggedPeg;
      return;
    }
  }

  if (brushTypeSelect.value === BRUSH_TYPES.SINGLE) {
    state.pegs.push({ x: state.startX, y: state.startY });
    drawCanvas();
  } else if (brushTypeSelect.value === BRUSH_TYPES.ERASER) {
    state.isErasing = true;
  } else {
    state.isDrawing = true;
  }
}

function onMouseMove(e) {
  const { x, y } = getCanvasCoordinates(e);
  updateCursor(x, y);

  if (state.isDragging && state.draggedPegIndex !== -1) {
    state.pegs[state.draggedPegIndex] = { x, y };
    drawCanvas();
  } else if (state.isDrawing) {
    drawCanvas();
    previewBrush(state.startX, state.startY, x, y);
  } else if (state.isErasing) {
    drawCanvas();
    previewEraseBox(state.startX, state.startY, x, y);
  }
}

function onMouseUp(e) {
  const { x: endX, y: endY } = getCanvasCoordinates(e);

  if (state.isDrawing) {
    drawBrush(state.startX, state.startY, endX, endY);
  } else if (state.isErasing) {
    eraseArea(state.startX, state.startY, endX, endY);
  }

  resetState();
  drawCanvas();
}

// Utility Functions
function getCanvasCoordinates(e) {
  const rect = canvas.getBoundingClientRect();
  return {
    x: Math.min(Math.max(e.clientX - rect.left, 0), canvas.width - 1),
    y: Math.min(Math.max(e.clientY - rect.top, 0), canvas.height - 1),
  };
}

function updateCursor(x, y) {
  canvas.style.cursor = isOverPeg(x, y) ? "pointer" : "crosshair";
}

function isOverPeg(x, y) {
  return state.pegs.some((peg) => Math.hypot(x - peg.x, y - peg.y) < 5);
}

function findDraggedPeg(x, y) {
  return state.pegs.findIndex((peg) => Math.hypot(x - peg.x, y - peg.y) < 5);
}

function resetState() {
  state.isDragging = false;
  state.isDrawing = false;
  state.isErasing = false;
  state.draggedPegIndex = -1;
}

// Brush Functions
function previewBrush(startX, startY, endX, endY) {
  const pegs = brushPegs(startX, startY, endX, endY);
  ctx.strokeStyle = "rgba(255, 0, 0, 0.5)";
  ctx.fillStyle = "rgba(255, 0, 0, 0.5)";

  pegs.forEach((peg) => {
    ctx.beginPath();
    ctx.arc(peg.x, peg.y, 5, 0, Math.PI * 2);
    ctx.fill();
  });

  ctx.beginPath();
  ctx.moveTo(pegs[0].x, pegs[0].y);
  pegs.slice(1).forEach((peg) => ctx.lineTo(peg.x, peg.y));
  ctx.closePath();
  ctx.stroke();
}

function brushPegs(startX, startY, endX, endY) {
  const pegCount = parseInt(pegCountInput.value);
  if (isNaN(pegCount) || pegCount < 2) return [];

  let shapeCoords;
  switch (brushTypeSelect.value) {
    case BRUSH_TYPES.LINE:
      shapeCoords = lineCoords(startX, startY, endX, endY, pegCount);
      break;
    case BRUSH_TYPES.BOX:
      shapeCoords = rectangleCoords(
        Math.min(startX, endX),
        Math.min(startY, endY),
        Math.abs(startX - endX),
        Math.abs(startY - endY),
        pegCount,
      );
      break;
    case BRUSH_TYPES.CIRCLE:
      const radius = Math.hypot(endX - startX, endY - startY);
      shapeCoords = circleCoords(startX, startY, radius, pegCount);
      break;
    default:
      return [];
  }

  const x_coords = shapeCoords.get_x();
  const y_coords = shapeCoords.get_y();

  // need to create a new array, for some reason the wasm one doesn't like map
  let out = [];
  x_coords.forEach((x, i) =>
    out.push({
      x: Math.min(x, canvas.width - 1),
      y: Math.min(y_coords[i], canvas.height - 1),
    }),
  );
  return out;
}

function drawBrush(startX, startY, endX, endY) {
  state.pegs = state.pegs.concat(brushPegs(startX, startY, endX, endY));
}

// Eraser Functions
function previewEraseBox(startX, startY, endX, endY) {
  ctx.strokeStyle = "rgba(255, 0, 0, 0.5)";
  ctx.strokeRect(
    Math.min(startX, endX),
    Math.min(startY, endY),
    Math.abs(endX - startX),
    Math.abs(endY - startY),
  );
}

function eraseArea(startX, startY, endX, endY) {
  if (Math.hypot(startX - endX, startY - endY) < 5) {
    erasePeg(startX, startY);
  } else {
    erasePegsInBox(
      Math.min(startX, endX),
      Math.min(startY, endY),
      Math.max(startX, endX),
      Math.max(startY, endY),
    );
  }
}

function erasePeg(x, y) {
  const index = state.pegs.findIndex(
    (peg) => Math.hypot(x - peg.x, y - peg.y) < 5,
  );
  if (index !== -1) {
    state.pegs.splice(index, 1);
  }
}

function erasePegsInBox(startX, startY, endX, endY) {
  state.pegs = state.pegs.filter(
    (peg) =>
      !(peg.x >= startX && peg.x <= endX && peg.y >= startY && peg.y <= endY),
  );
}

function clearPegs() {
  state.pegs = [];
  drawCanvas();
}

function autoPegs() {
  let peg_coords;
  const pegCount = parseInt(pegCountInput.value);
  if (brushTypeSelect.value == BRUSH_TYPES.BOX) {
    // add pegs around the edge of the image
    peg_coords = rectangleCoords(
      0,
      0,
      canvas.width - 1,
      canvas.height - 1,
      pegCount,
    );
  } else {
    // add pegs in a circle and centered on the image
    const radius = Math.min(canvas.width, canvas.height) / 2;
    const center_x = canvas.width / 2;
    const center_y = canvas.height / 2;
    peg_coords = circleCoords(center_x, center_y, radius, pegCount);
  }
  const x_coords = peg_coords.get_x();
  const y_coords = peg_coords.get_y();
  // need to create a new array, for some reason the wasm one doesn't like map
  let out = [];
  x_coords.forEach((x, i) =>
    out.push({
      x: Math.min(x, canvas.width - 1),
      y: Math.min(y_coords[i], canvas.height - 1),
    }),
  );
  state.pegs = state.pegs.concat(out);
  drawPegs();
}

// Initialize the application
initialize();
