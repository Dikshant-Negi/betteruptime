import { Link, useNavigate } from "react-router-dom";
import Input from "../store/Input";
import { useEffect, useRef, useState } from "react";
import logo from "../../assets/logo.png";
import { authLogin } from "../../api/api";

export default function SignIn() {
  const navigate = useNavigate();
  let ref = useRef<HTMLInputElement>(null);
  let [active, setIsActive] = useState<boolean>(false);
  let [isLoading, setIsLoading] = useState<boolean>(false);
  let [payload, setPayLoad] = useState<{
    email: string | null;
    password: string | null;
  }>({ email: null, password: null });

  useEffect(() => {
    ref.current?.focus();
  });
  
  const handleLogin = async () => {
    if(!active) {
      if(payload.email) setIsActive(true);
      return;
    }

    if(payload.email && payload.password) {
      setIsLoading(true);
      try {
        const response = await authLogin(payload.email, payload.password);

        if(response.success) {
          localStorage.setItem('token', response.jwt);
          navigate('/dashboard'); //later /dashboard
        }
      } catch (err) {
        console.error("Login failed:", err);
        alert("Invalid credentials");
      } finally {
        setIsLoading(false);
      }
    }
  }

  return (
    <div className="min-h-screen w-full flex items-center justify-center bg-primary-100">
      <div className="w-full max-w-md flex flex-col items-center gap-6 text-white p-8">
        <div className="mb-2">
          <img src={logo} alt="Logo" className="h-20 w-20 rounded-full object-contain bg-black mx-auto mb-6" />
        </div>

        <h1 className="text-3xl font-semibold">Welcome back</h1>

        <div className="flex gap-1 text-sm text-slate-400">
          <span>First time here?</span>
          <Link to="/signup" className="text-blue-400 hover:underline">
            Sign up
          </Link>
        </div>

        <div className="w-full flex flex-col gap-4 mt-4">
          {!active ? (
            <Input
              ref={ref}
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
            />
          ) : (
            <Input
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
            />
          )}
        </div>

        <button
          className="w-full mt-4 py-3 rounded-xl bg-indigo-500 cursor-pointer hover:bg-indigo-600 transition text-white font-medium"
          onClick={handleLogin}
          disabled={isLoading}
        >
          {isLoading ? "Signing in..." : active ? "Sign in" : "Next"}
        </button>

        <p className="text-xs text-slate-500 text-center mt-6">
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
