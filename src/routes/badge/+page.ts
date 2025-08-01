import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = ({ params }) => {
  return {
    id: 1,
    name: "John Doe",
    email: "john.doe@example.com",
  };
}; 