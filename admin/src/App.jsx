import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import AddBook from "./components/AddBook";
import AddChapters from "./components/AddChapters";
import Nav from "./components/Nav";


const App = () => {
  return (
    <div className="App">
      <Router>
        <Nav />
        <Routes>
          <Route path="/add-book" element={<AddBook />} />
          <Route path="/add-chapter" element={<AddChapters />} />
        </Routes>
      </Router>
    </div>
  )
}

export default App
