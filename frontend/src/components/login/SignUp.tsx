import { Link, useNavigate } from "react-router-dom";
import Input from "../store/Input";
import { useEffect, useRef, useState } from "react";
import StepDots from "../store/StepDots";
import { validation } from "../../utility/extra";
import { authRegister } from "../../api/api";
import { useMutation  } from "@tanstack/react-query";
import Logo from "../../assets/Logo";

export default function SignUp() {
  const navigate = useNavigate();
  const signupMutation = useMutation({
    mutationFn: (data: 
      {
        email: string, password:string, username:string
      }) => authRegister(data.email, data.password, data.username),

      onSuccess: (response) => {
        if(response.success){
          localStorage.setItem("token", response.jwt);
          console.log("user added successfully");
          navigate("/dashboard");
        }
      }, 
      onError: () => {
        alert("Email already exist go to Sign in");
        navigate("/signin");
      },

  });
  let ref = useRef<{
    username: HTMLInputElement | null;
    email: HTMLInputElement | null;
    password: HTMLInputElement | null;
  }>({ username: null, email: null, password: null });

  let [step, setStep] = useState<number>(1);

  let [payload, setPayLoad] = useState<{
    username: string | null;
    email: string | null;
    password: string | null;
  }>({ username: null, email: null, password: null });

  let [error, setError] = useState<{
    username: string;
    email: string;
    password: string;
  }>({ username: "", email: "", password: "" });

  useEffect(() => {
    if (step == 1) {
      ref.current?.username?.focus();
    } else if (step == 2) {
      ref.current?.email?.focus();
    } else if (step == 3) {
      ref.current?.password?.focus();
    }
  }, [step]);

  const handleSubmit = async(e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (step <= 3) {
      if (step == 1 && !validation(payload.username, "username", setError)) {
        setError(
          (prev: { username: string; email: string; password: string }) => ({
            ...prev,
            username: "Atleast 4 characters with letters",
          }),
        );
        return;
      } else if (step == 2 && !validation(payload.email, "email", setError)) {
        setError(
          (prev: { username: string; email: string; password: string }) => ({
            ...prev,
            email: "Please enter a valid email address",
          }),
        );
        return;
      } else if (
        step == 3 &&
        !validation(payload.password, "password", setError)
      ) {
        setError(
          (prev: { username: string; email: string; password: string }) => ({
            ...prev,
            password:
              "Password cannot be empty and must be at least 6 characters",
          }),
        );
        return;
      }
    }

    if (step <= 3) {
      if (step == 1 && validation(payload.username, "username", setError)) {
        setError(
          (prev: { username: string; email: string; password: string }) => ({
            ...prev,
            username: "",
          }),
        );
        setStep(step + 1);
      } else if (
        step == 2 &&
        validation(payload.username, "username", setError)
      ) {
        setError(
          (prev: { username: string; email: string; password: string }) => ({
            ...prev,
            email: "",
          }),
        );
        setStep(step + 1);
      } else {
        setError(
          (prev: { username: string; email: string; password: string }) => ({
            ...prev,
            password: "",
          }),
        );
        if(payload.email && payload.password && payload.username){
          signupMutation.mutate({
            email: payload.email,
            password: payload.password,
            username: payload.password,
          });
        }
      }
    }
  };

  return (
    <div className="min-h-screen w-full flex items-center justify-center bg-primary-100">
      <div className="w-full max-w-md flex flex-col items-center gap-6 text-white p-8">
        <div className="mb-2">
          < Logo className="h-20 w-20q"   />
        </div>

        <h1 className="text-3xl font-semibold">Sign up for free</h1>

        <div className="flex gap-1 text-sm text-slate-400">
          <span>Already have an account?</span>
          <Link to="/signin" className="text-blue-400 hover:underline">
            Sign in
          </Link>
        </div>

        <form
          onSubmit={handleSubmit}
          className="w-full flex flex-col gap-4 mt-4"
        >
          {step == 1 ? (
            <Input
              ref={(el) => {
                if (ref.current) ref.current.username = el;
              }}
              label="Username"
              inputName="username"
              type="text"
              text={payload.username != null ? payload.username : ""}
              inputProp="h-10 px-3 rounded-md  "
              placeholder="Your username"
              onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                setPayLoad({
                  ...payload,
                  username: (e.target as HTMLInputElement).value as string,
                })
              }
              pattern="(?=.*[a-z]).{4,}"
              error={error.username}
            />
          ) : step == 2 ? (
            <Input
              ref={(el) => {
                if (ref.current) ref.current.email = el;
              }}
              label="E-mail"
              inputName="email"
              type="email"
              text={payload.email != null ? payload.email : ""}
              inputProp="h-10 px-3 rounded-md  "
              placeholder="Your work e-mail"
              onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                setPayLoad({
                  ...payload,
                  email: (e.target as HTMLInputElement).value as string,
                })
              }
              error={error.email}
            />
          ) : (
            <Input
              ref={(el) => {
                if (ref.current) ref.current.password = el;
              }}
              text={payload.password != null ? payload.password : ""}
              type="text"
              label="Password"
              inputName="password"
              inputProp="h-10 px-3 rounded-md  "
              placeholder="password"
              onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                setPayLoad({
                  ...payload,
                  password: (e.target as HTMLInputElement).value as string,
                })
              }
              error={error.password}
            />
          )}
          <button
            className="w-full mt-4 py-3 rounded-xl bg-indigo-500 cursor-pointer hover:bg-indigo-600 transition text-white font-medium"
            type="submit"
            disabled={signupMutation.isPending}
          >
            {signupMutation.isPending ? "Creating Account..." : (step == 3 ? "Sign Up" : "Next")}
          </button>
        </form>

        <StepDots setStep={setStep} n={3} step={step} />

        <p className="text-xs text-slate-500 text-center pt-6">
          You acknowledge that you read, and agree to our{" "}
          <Link to="/terms" className="text-blue-400 hover:underline">
            Terms of Service
          </Link>{" "}
          and our{" "}
          <Link to="/privacy" className="text-blue-400 hover:underline">
            Privacy Policy
          </Link>
          .
        </p>
      </div>
    </div>
  );
}
