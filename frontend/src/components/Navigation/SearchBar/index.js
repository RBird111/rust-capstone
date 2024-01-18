import "./SearchBar.scss";

const SearchBar = () => {
  return (
    <div className="search-bar">
      <input
        type="text"
        placeholder="Search local businesses..."
        onFocus={() => {
          const div = document.querySelector("div.search-bar");
          div.id = "active-search";
        }}
        onBlur={() => {
          const div = document.querySelector("div.search-bar");
          div.id = "";
        }}
      />
      <i
        className="fa-solid fa-magnifying-glass"
        onClick={() => {
          alert("Search feature coming soon!");
        }}
      />
    </div>
  );
};

export default SearchBar;
