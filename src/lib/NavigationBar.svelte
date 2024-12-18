<nav>
	<a href="/">
		<h1>Colorful Ambience</h1>
	</a>

	<ul>
		<li><a href="/app">App</a></li>
	</ul>
</nav>

<style>
nav {
	display: flex;
	padding: 1rem 5rem;
	border-bottom: 2px solid #FFD8B8;
}

h1 {
	font-size: 2rem;
	font-weight: 900;
	margin-right: 1rem;
}

ul li {
	padding: 0 1rem;
	list-style: none;
	font-size: 1.2rem;
	font-weight: 600;
}

@media(max-width: 600px) {
	nav {
		padding: 1rem 2rem;
	}

	h1 {
		font-size: 1.6rem;
	}
}
@media(max-width: 400px) {
	h1 {
		font-size: 1.4rem;
	}

	ul li {
		padding: 0 0.6rem;
	}
}

a {
	color: #552D2D;
	text-decoration: none;
}

ul {
	display: flex;
	align-items: center;
}
</style>
