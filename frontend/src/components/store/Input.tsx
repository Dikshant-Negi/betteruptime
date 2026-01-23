import React, { forwardRef } from "react";

type InputProps = {
  label: string;
  labelProps?: string;
  inputName: string;
  inputProp?: string;
  placeholder?: string;
  onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void;
  text: string;
  type?: string;
};

const Input = forwardRef<HTMLInputElement, InputProps>(
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
          className={`${inputProp} border-gray-300 focus:border-white focus:ring-2 focus:ring-white focus:outline-none transition-all duration-200`}
        />
      </>
    );
  }
);

export default Input;
