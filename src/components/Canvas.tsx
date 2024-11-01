import React, {
  Dispatch,
  forwardRef,
  SetStateAction,
  useCallback,
  useEffect,
  useRef,
  useState,
} from "react";
import { BrushType } from "../types/brushType";
import { lineCoords, rectangleCoords, circleCoords } from "strandify-wasm";

interface CanvasProps {
  image: HTMLImageElement | null;
  pegs: Array<{ x: number; y: number }>;
  setPegs: Dispatch<SetStateAction<Array<{ x: number; y: number }>>>;
  brushType: BrushType;
  pegCount: number;
}

interface Position {
  x: number;
  y: number;
}

interface SelectionBox {
  x: number;
  y: number;
  width: number;
  height: number;
}

export const Canvas = forwardRef<HTMLCanvasElement, CanvasProps>(
  ({ image, pegs, setPegs, brushType, pegCount }, ref) => {
    // State management with more explicit types
    const [mouseState, setMouseState] = useState<{
      isDragging: boolean;
      draggedPegIndex: number;
      isDrawing: boolean;
      isErasing: boolean;
    }>({
      isDragging: false,
      draggedPegIndex: -1,
      isDrawing: false,
      isErasing: false,
    });

    const [startPos, setStartPos] = useState<Position>({ x: 0, y: 0 });
    const [selectionBox, setSelectionBox] = useState<SelectionBox | null>(null);
    const [previewPegs, setPreviewPegs] = useState<Array<Position>>([]);

    const contextRef = useRef<CanvasRenderingContext2D | null>(null);
    const containerRef = useRef<HTMLDivElement>(null);

    // Memoized brush pegs generation
    const generateBrushPegs = useCallback(
      (
        startX: number,
        startY: number,
        endX: number,
        endY: number,
      ): Array<Position> => {
        if (!ref || !("current" in ref) || !ref.current) return [];
        if (isNaN(pegCount) || pegCount < 2) return [];

        let shapeCoords;
        switch (brushType) {
          case BrushType.Line:
            shapeCoords = lineCoords(startX, startY, endX, endY, pegCount);
            break;
          case BrushType.Box:
            shapeCoords = rectangleCoords(
              Math.min(startX, endX),
              Math.min(startY, endY),
              Math.abs(startX - endX),
              Math.abs(startY - endY),
              pegCount,
            );
            break;
          case BrushType.Circle:
            shapeCoords = circleCoords(
              startX,
              startY,
              Math.hypot(endX - startX, endY - startY),
              pegCount,
            );
            break;
          default:
            return [];
        }

        const x_coords = shapeCoords.get_x();
        const y_coords = shapeCoords.get_y();

        const out: Array<Position> = [];
        x_coords.forEach((x, i) =>
          out.push({
            x: Math.min(x, ref.current!.width - 1),
            y: Math.min(y_coords[i], ref.current!.height - 1),
          }),
        );
        return out;
      },
      [brushType, pegCount, ref],
    );

    // Memoized preview brush function
    const previewBrush = useCallback(
      (startX: number, startY: number, endX: number, endY: number) => {
        setPreviewPegs(generateBrushPegs(startX, startY, endX, endY));
      },
      [generateBrushPegs],
    );

    // Resize canvas and move pegs to new positions
    const resizeCanvas = useCallback(() => {
      if (!ref || !("current" in ref) || !ref.current || !containerRef.current)
        return;

      const canvas = ref.current;
      const container = containerRef.current;
      const maxWidth = container.clientWidth;
      const maxHeight = window.innerHeight * 0.6;

      let width, height;

      if (image) {
        const imageAspectRatio = image.width / image.height;
        const containerAspectRatio = maxWidth / maxHeight;

        if (imageAspectRatio > containerAspectRatio) {
          width = maxWidth;
          height = maxWidth / imageAspectRatio;
        } else {
          height = maxHeight;
          width = maxHeight * imageAspectRatio;
        }
      } else {
        width = maxWidth;
        height = maxWidth * (9 / 16);
      }

      // Only update if dimensions have actually changed
      if (canvas.width !== width || canvas.height !== height) {
        const scaleX = width / canvas.width;
        const scaleY = height / canvas.height;

        canvas.width = width;
        canvas.height = height;

        // Use functional update to avoid stale closure
        setPegs((currentPegs) =>
          currentPegs.map((peg) => ({
            x: peg.x * scaleX,
            y: peg.y * scaleY,
          })),
        );
      }
    }, [ref, image, setPegs]);

    // Event Handlers with improved type safety
    const handleMouseDown = useCallback(
      (e: React.MouseEvent<HTMLCanvasElement>) => {
        if (!ref || !("current" in ref) || !ref.current) return;

        const canvas = ref.current;
        const rect = canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;

        setStartPos({ x, y });

        if (brushType === BrushType.Eraser) {
          setMouseState((prev) => ({ ...prev, isErasing: true }));
          setSelectionBox({ x, y, width: 0, height: 0 });
          return;
        }

        const draggedPeg = pegs.findIndex(
          (peg) => Math.hypot(x - peg.x, y - peg.y) < 5,
        );

        if (draggedPeg !== -1) {
          setMouseState((prev) => ({
            ...prev,
            isDragging: true,
            draggedPegIndex: draggedPeg,
          }));
          return;
        }

        if (brushType === BrushType.Single) {
          setPegs([...pegs, { x, y }]);
        } else {
          setMouseState((prev) => ({ ...prev, isDrawing: true }));
        }
      },
      [brushType, pegs, ref, setPegs],
    );

    const handleMouseMove = useCallback(
      (e: React.MouseEvent<HTMLCanvasElement>) => {
        if (!ref || !("current" in ref) || !ref.current) return;

        const canvas = ref.current;
        const rect = canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;

        const { isDragging, draggedPegIndex, isErasing, isDrawing } =
          mouseState;

        if (isDragging && draggedPegIndex !== -1) {
          const newPegs = [...pegs];
          newPegs[draggedPegIndex] = { x, y };
          setPegs(newPegs);
        } else if (isErasing && selectionBox) {
          setSelectionBox({
            x: Math.min(startPos.x, x),
            y: Math.min(startPos.y, y),
            width: Math.abs(x - startPos.x),
            height: Math.abs(y - startPos.y),
          });
        } else if (isDrawing) {
          previewBrush(startPos.x, startPos.y, x, y);
        }
      },
      [mouseState, pegs, startPos, selectionBox, ref, setPegs, previewBrush],
    );

    const handleMouseUp = useCallback(
      (e: React.MouseEvent<HTMLCanvasElement>) => {
        if (!ref || !("current" in ref) || !ref.current) return;
        const canvas = ref.current;
        const rect = canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;

        const { isErasing, isDrawing } = mouseState;

        if (isErasing && brushType === BrushType.Eraser && selectionBox) {
          const newPegs = pegs.filter((peg) => {
            return !(
              peg.x >= selectionBox.x &&
              peg.x <= selectionBox.x + selectionBox.width &&
              peg.y >= selectionBox.y &&
              peg.y <= selectionBox.y + selectionBox.height
            );
          });
          setPegs(newPegs);
        } else if (isDrawing) {
          setPegs([
            ...pegs,
            ...generateBrushPegs(startPos.x, startPos.y, x, y),
          ]);
        }

        // Reset mouse state
        setSelectionBox(null);
        setMouseState({
          isDragging: false,
          draggedPegIndex: -1,
          isDrawing: false,
          isErasing: false,
        });
        setPreviewPegs([]);
      },
      [
        mouseState,
        brushType,
        pegs,
        startPos,
        selectionBox,
        ref,
        setPegs,
        generateBrushPegs,
      ],
    );

    // Lifecycle and side effects
    // draw the pegs on the canvas when anything changes
    useEffect(() => {
      if (ref === null || !("current" in ref) || ref.current === null) return;

      const canvas = ref.current;
      const ctx = canvas.getContext("2d");
      if (!ctx) return;

      ctx.clearRect(0, 0, canvas.width, canvas.height);

      if (image) {
        ctx.drawImage(image, 0, 0, canvas.width, canvas.height);
      }

      if (selectionBox) {
        ctx.fillStyle = "rgba(255, 0, 0, 0.2)";
        ctx.fillRect(
          selectionBox.x,
          selectionBox.y,
          selectionBox.width,
          selectionBox.height,
        );
        ctx.strokeStyle = "rgba(255, 0, 0, 0.8)";
        ctx.strokeRect(
          selectionBox.x,
          selectionBox.y,
          selectionBox.width,
          selectionBox.height,
        );
      }

      pegs.forEach((peg) => {
        ctx.beginPath();
        ctx.arc(peg.x, peg.y, 5, 0, Math.PI * 2);
        ctx.fillStyle = "red";
        ctx.fill();
      });

      if (previewPegs.length > 0) {
        ctx.strokeStyle = "rgba(255, 0, 0, 0.5)";
        ctx.fillStyle = "rgba(255, 0, 0, 0.5)";

        previewPegs.forEach((peg) => {
          ctx.beginPath();
          ctx.arc(peg.x, peg.y, 5, 0, Math.PI * 2);
          ctx.fill();
        });

        if (previewPegs.length > 1) {
          ctx.beginPath();
          ctx.moveTo(previewPegs[0].x, previewPegs[0].y);
          previewPegs.slice(1).forEach((peg) => ctx.lineTo(peg.x, peg.y));
          ctx.closePath();
          ctx.stroke();
        }
      }
    }, [ref, image, pegs, selectionBox, previewPegs]);

    useEffect(() => {
      resizeCanvas();
    }, [resizeCanvas]);

    useEffect(() => {
      if (ref && "current" in ref && ref.current) {
        contextRef.current = ref.current.getContext("2d");
        resizeCanvas();
      }
    }, [ref, image, resizeCanvas]);

    useEffect(() => {
      window.addEventListener("resize", resizeCanvas);
      return () => window.removeEventListener("resize", resizeCanvas);
    }, [resizeCanvas]);

    return (
      <div ref={containerRef} className="w-full" role="region">
        <canvas
          ref={ref}
          className="mx-auto border border-gray-200 dark:border-gray-600 cursor-crosshair"
          onMouseDown={handleMouseDown}
          onMouseMove={handleMouseMove}
          onMouseUp={handleMouseUp}
          onMouseLeave={handleMouseUp}
        />
      </div>
    );
  },
);

Canvas.displayName = "Canvas";
