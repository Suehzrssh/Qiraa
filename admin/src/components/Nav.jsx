import {Link} from "react-router-dom";

const Nav = () => {
  return (
    <nav className="w-full flex flex-row justify-center items-center gap-12">
        <Link to="/add-book">Add Book</Link>
        <Link to="/add-chapter">Add Chapter</Link>
    </nav>
  )
}

export default Nav
