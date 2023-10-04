
<script>
import { graphql } from "./gql"

const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
</script>
