import { BrowserRouter, Routes, Route } from "react-router-dom";
import SignUp from "./components/login/SignUp";


function App() {
 

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/login" element={<SignUp />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
