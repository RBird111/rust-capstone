import { NavLink, useParams } from "react-router-dom";
import { useDispatch, useSelector } from "react-redux";
import { useEffect, useState } from "react";

import "./BusinessFeed.scss";
import { getAllBusinesses } from "../../store/business";
import LoadingIcon from "../FormElements/LoadingIcon";
import BusinessFeedItem from "./BusinessFeedItem";

const BusinessFeed = () => {
  const dispatch = useDispatch();

  const { category } = useParams();
  const businesses = useSelector((state) => state.business.allBusinesses);
  // const categoryBusinesses = Object.values(businesses).filter(
  //   (business) => business.category === category
  // );
  const categoryBusinesses = Object.values(businesses);

  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    dispatch(getAllBusinesses(category)).then(() => setIsLoaded(true));
  }, [category, dispatch]);

  const iconClass = {
    restaurant: "fa-utensils",
    shopping: "fa-cart-shopping",
    automotive: "fa-car",
    "home services": "fa-house",
  };

  if (!isLoaded) return <LoadingIcon />;

  return (
    <>
      <h1 className="business-feed-title">
        <i className={`fa-solid ${iconClass[category]} fa-lg`} />
        {category
          .split(" ")
          .map((word) => word[0].toUpperCase() + word.slice(1))
          .join(" ")}{" "}
      </h1>

      <div className="business-feed">
        {categoryBusinesses.reverse().map((business) => (
          <NavLink
            key={business.id}
            className="b-feed-link"
            to={`/business/${business.id}`}
          >
            <BusinessFeedItem business={business} />
          </NavLink>
        ))}
      </div>
    </>
  );
};

export default BusinessFeed;
