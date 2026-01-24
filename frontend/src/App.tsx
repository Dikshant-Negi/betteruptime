import { BrowserRouter, Routes, Route } from "react-router-dom";
import SignUp from "./components/login/SignUp";
import SignIn from "./components/login/SignIn";


function App() {
 

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/signup" element={<SignUp />} />
        <Route path="/signin" element={<SignIn />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
