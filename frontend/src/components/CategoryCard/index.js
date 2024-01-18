import { NavLink } from "react-router-dom";
import "./CategoryCard.scss";

const CategoryCard = ({ category, url }) => {
  const iconClass = {
    restaurant: "fa-utensils",
    shopping: "fa-cart-shopping",
    automotive: "fa-car",
    "home services": "fa-house",
  };

  return (
    <NavLink className="category-wrap" to={`/category/${category}`}>
      <div
        className="background"
        style={{
          backgroundImage: `url(${url})`,
        }}
      />

      <div className="category-card">
        <i className={`fa-solid ${iconClass[category]} fa-lg`} />

        <p className="category-name">
          {category
            .split(" ")
            .map((word) => word[0].toUpperCase() + word.slice(1))
            .join(" ")}
        </p>
      </div>
    </NavLink>
  );
};

export default CategoryCard;
