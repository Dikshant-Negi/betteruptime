import type { Component } from "lucide-react";
import React, { forwardRef, type JSX } from "react";

type InputProps = {
  label: string;
  labelProps?: string;
  inputName: string;
  inputProp?: string;
  placeholder?: string;
  onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void;
  text: string;
  type?: string;
  pattern?:string
  error?:string
 
};

const Input = forwardRef<HTMLInputElement | null, InputProps>(
  (
    {
      label,
      labelProps,
      inputName,
      inputProp,
      placeholder,
      onChange,
      text,
      type = "text",
      pattern ,
      error="",
  
    },
    ref
  ) => {
    return (
      <>
        <label className={labelProps}>{label}</label>
        <input
          ref={ref}
          name={inputName}
          type={type}
          value={text}
          placeholder={placeholder}
          onChange={onChange}
          className={`${inputProp}  border-gray-300 focus:border-white focus:ring-2 focus:ring-white focus:outline-none transition-all duration-200`}
          {...(pattern ? { pattern } : {})}
        />
    
        {error.trim() && <p className="text-red-500 text-sm mt-1">{error}</p>}
      </>
    );
  }
);

export default Input;