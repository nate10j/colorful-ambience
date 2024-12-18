<script>
import init, {greet} from "noise-generator"

init().then(() => {
	console.log('init wasm-pack');
	greet('from vite!');
});
</script>

<div class="container">

</div>
