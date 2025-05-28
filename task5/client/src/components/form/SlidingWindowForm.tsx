import { useForm, type SubmitHandler } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { ErrorMessage } from "@hookform/error-message";
import { type Dispatch, type FC, type SetStateAction } from "react";
import {
  slidingWindowSchema,
  type SlidingWindowSchemaType,
} from "../../validations/slidingWindow";
import type { SlidingWindowDataType } from "../../model/slidingWindowData";

type Props = {
  types: "count" | "time";
  setResult: Dispatch<SetStateAction<SlidingWindowDataType | null>>;
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
    // create object
    const socket = new WebSocket("ws://localhost:7000");
    // connect server
    socket.addEventListener("open", (_) => {
      socket.send(`${props.types},${formData.window},${formData.slide}`);
    });
    // listen for message
    socket.addEventListener("message", (event) => {
      const data: SlidingWindowDataType = JSON.parse(event.data);
      console.table(data);
      props.setResult(data);
    });
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
