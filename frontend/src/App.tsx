import { BrowserRouter, Routes, Route } from "react-router-dom";
import SignUp from "./components/login/SignUp";
import SignIn from "./components/login/SignIn";
import DashBoard from "./components/dashboard/Dashboard";
import { PrivateRoute } from "./components/auth/PrivateRoute";
import LandingPage from "./components/landingPage/LandingPange";

function App() {
 

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<LandingPage />} />
        <Route path="/signup" element={<SignUp />} />
        <Route path="/signin" element={<SignIn />} />
        
        <Route path="/dashboard" element={
          <PrivateRoute>
            <DashBoard/>
          </PrivateRoute>
          } 
        />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
