import { useDispatch, useSelector } from "react-redux";
import { useEffect, useState } from "react";

import "./ReviewForm.scss";
import { useModal } from "../../context/Modal";
import {
  createReview,
  deleteReview,
  getReview,
  updateReview,
} from "../../store/reviews";
import DefaultButton from "../FormElements/DefaultButton";
import StarRatingBar from "../FormElements/StarRatingBar";
import Error from "../FormElements/Error";
import { getBusiness } from "../../store/business";
import ConfirmDelete from "../FormElements/ConfirmDelete";

const ReviewForm = ({ business, review }) => {
  const dispatch = useDispatch();

  const { setModalContent, closeModal } = useModal();
  const reviewData = useSelector((state) => state.reviews.currReview);

  const [rating, setRating] = useState(review ? review.rating : null);
  const [body, setBody] = useState(review ? review.body : "");
  const [errors, setErrors] = useState({});
  const [isSubmitted, setIsSubmitted] = useState(false);

  useEffect(() => {
    if (review) dispatch(getReview(review.id));
  }, [dispatch, review]);

  // Validations
  useEffect(() => {
    setErrors({});
    const errorsObj = {};

    if (!rating) errorsObj.rating = "Must give a rating";

    if (!body) errorsObj.body = "Must fill out review";
    else if (body.length > 1000)
      errorsObj.body = "Review must be less than 1000 characters";

    setErrors(errorsObj);
  }, [body, rating]);

  const handleSubmit = async (e) => {
    e.preventDefault();

    setIsSubmitted(true);

    if (Object.values(errors).length === 0) {
      const formData = review ? { ...reviewData } : {};

      formData.rating = rating;
      formData.body = body;
      formData.business_id = business.id;

      let data;
      if (review) {
        data = await dispatch(updateReview(formData));
      } else {
        data = await dispatch(createReview(formData));
      }

      if (data.errors) {
        const errorsObj = {};

        for (const error of data.errors) {
          const [name, message] = error.split(" : ");
          errorsObj[name] = message;
        }

        return setErrors(errorsObj);
      }

      await dispatch(getBusiness(business.id));
      closeModal();
    }
  };

  return (
    <div className="review-form">
      {review && (
        <div
          className="delete"
          title="Click to delete your review"
          onClick={() =>
            setModalContent(
              <ConfirmDelete
                item={review}
                thunk={deleteReview}
                business={business}
              />
            )
          }
        >
          <i className="fa-solid fa-trash" />
        </div>
      )}

      <h1>{business.name}</h1>
      <p className="p-title">How would you rate your experience?</p>

      <form onSubmit={handleSubmit}>
        {isSubmitted && errors.rating && <Error error={errors.rating} />}
        <StarRatingBar rating={rating} setRating={setRating} />

        {isSubmitted && errors.body && <Error error={errors.body} />}
        <textarea
          placeholder="Write your review..."
          value={body}
          onChange={(e) => setBody(e.target.value)}
        />

        <DefaultButton text={`${review ? "Update" : "Submit"} Review`} />
      </form>
    </div>
  );
};

export default ReviewForm;
