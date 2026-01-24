import type { JSX } from "react";

interface StepDotsProps {
  setStep: React.Dispatch<React.SetStateAction<number>>;
  n: number;
  step: number;
}
export default function StepDots({
  setStep,
  n,
  step,
}: StepDotsProps): JSX.Element {
  return (
    <div>
      {Array.from({ length: n }, (_, index) => {
        return (
          <span
            onClick={() => setStep(index + 1)}
            className={`h-3 w-3 rounded-full inline-block mx-1 cursor-pointer ${index + 1 == step ? "bg-indigo-500" : "bg-white"}`}
          ></span>
        );
      })}
    </div>
  );
}
