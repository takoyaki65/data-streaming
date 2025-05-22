import { useForm, type SubmitHandler } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { ErrorMessage } from "@hookform/error-message";
import type { FC } from "react";
import {
  slidingWindowSchema,
  type SlidingWindowSchemaType,
} from "../../validations/slidingWindow";

type Props = {
  types: "Count" | "Time";
};

const SlidingWindowForm: FC<Props> = (props) => {
  // define form schema
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<SlidingWindowSchemaType>({
    resolver: zodResolver(slidingWindowSchema),
    defaultValues: {},
  });

  // send to middle-server
  const onSubmit: SubmitHandler<SlidingWindowSchemaType> = async (formData) => {
    console.table(formData);
  };

  return (
    <div>
      <h1>{props.types} based sliding window</h1>
      <form onSubmit={handleSubmit(onSubmit)}>
        <label>Window Size</label>
        <input
          id="window"
          type="number"
          step="any"
          {...register("window", { valueAsNumber: true })}
        />
        <br />
        <ErrorMessage
          errors={errors}
          name="window"
          message={errors.window?.message}
        />
        <br />
        <label>Slide Size</label>
        <input
          id="slide"
          type="number"
          step="any"
          {...register("slide", { valueAsNumber: true })}
        />
        <br />
        <ErrorMessage
          errors={errors}
          name="slide"
          message={errors.slide?.message}
        />
        <br />
        <input type="submit" value="送信" />
      </form>
    </div>
  );
};

export default SlidingWindowForm;
